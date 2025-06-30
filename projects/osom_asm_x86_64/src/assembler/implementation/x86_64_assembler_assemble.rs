#![allow(
    unused_unsafe,
    clippy::checked_conversions,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_ptr_alignment,
    clippy::unnecessary_wraps
)]

use std::collections::HashMap;

use osom_encoders_x86_64::encoders as enc;
use osom_encoders_x86_64::models as enc_models;

use crate::assembler::implementation::fragment::FragmentOrderId;
use crate::assembler::implementation::fragment::RelaxationVariant;
use crate::assembler::implementation::fragment::const_sizes;
use crate::assembler::implementation::macros::fragment_end;
use crate::assembler::{AssembleError, EmissionData};
use crate::models::Condition;
use crate::models::Label;

use super::macros::{fragment_at_index, fragment_at_index_mut};
use super::{X86_64Assembler, fragment::Fragment};

impl X86_64Assembler {
    pub fn assemble(mut self, stream: &mut dyn std::io::Write) -> Result<EmissionData, AssembleError> {
        let mut offsets = calculate_initial_offsets(&self)?;
        relax_instructions_and_update_offsets(&mut self, &mut offsets)?;
        let labels_map = calculate_labels_map(&self, &offsets)?;
        patch_addresses(&mut self, &labels_map, &offsets)?;
        emit_fragments(&self, &labels_map, &offsets, stream)
    }
}

fn calculate_initial_offsets(asm: &X86_64Assembler) -> Result<HashMap<FragmentOrderId, u32>, AssembleError> {
    let mut result = HashMap::with_capacity(asm.fragments_count as usize);

    let start = fragment_at_index!(asm, 0) as *const Fragment;
    let end = fragment_end!(asm);
    let get_id = |fragment: *const Fragment| -> FragmentOrderId {
        let u8_fragment = fragment.cast::<u8>();
        let u8_start = start.cast::<u8>();
        let ptr_diff = unsafe { u8_fragment.offset_from(u8_start) };
        FragmentOrderId::from_index(ptr_diff as u32)
    };

    let mut current_fragment = start;
    let mut current_offset = 0;
    let fragment_id = get_id(current_fragment);
    result.insert(fragment_id, current_offset);

    while current_fragment < end {
        let current_fragment_ref = unsafe { &*current_fragment };
        current_offset += current_fragment_ref.data_length();
        current_fragment = unsafe { current_fragment_ref.next() };
        result.insert(get_id(current_fragment), current_offset);
    }

    Ok(result)
}

/// I really don't know where this shift by 3 comes from.
/// But it is needed, so I won't dive deep into it.
const MAGIC_SHIFT: isize = 3;

fn relax_instructions_and_update_offsets(
    asm: &mut X86_64Assembler,
    offsets: &mut HashMap<FragmentOrderId, u32>,
) -> Result<(), AssembleError> {
    let start = fragment_at_index_mut!(asm, 0) as *mut Fragment;
    let end = fragment_end!(asm);

    let get_id = |fragment: *const Fragment| -> FragmentOrderId {
        let u8_fragment = fragment.cast::<u8>();
        let u8_start = start.cast::<u8>();
        let ptr_diff = unsafe { u8_fragment.offset_from(u8_start) };
        FragmentOrderId::from_index(ptr_diff as u32)
    };

    let get_position = |label: &Label, offsets: &HashMap<FragmentOrderId, u32>| -> Result<u32, AssembleError> {
        let Some(label_offset) = asm.label_offsets.get(label) else {
            return Err(AssembleError::LabelNotSet(*label));
        };

        let fragment_index = label_offset.fragment_id.index();
        let fragment = fragment_at_index!(asm, fragment_index);

        let relaxation_offset = match fragment {
            Fragment::Bytes { .. } => 0,
            _ => fragment.data_length(),
        };

        let fragment_offset = offsets.get(&label_offset.fragment_id).unwrap();
        Ok(*fragment_offset + relaxation_offset + label_offset.in_fragment_offset)
    };

    macro_rules! update_subsequent_offsets {
        ($start:expr, $add:expr) => {{
            let start: *mut Fragment = $start;
            let add: u32 = $add;
            let mut current = unsafe { (*start).next() };
            while current.cast_const() < end {
                let current_id = get_id(current);
                *offsets.get_mut(&current_id).unwrap() += add;
                current = unsafe { (*current).next() };
            }
        }};
    }

    loop {
        let mut has_changes = false;

        let mut current_fragment = start;
        while current_fragment.cast_const() < end {
            let current_fragment_ref = unsafe { &mut *current_fragment };
            if let Fragment::Bytes { .. } = current_fragment_ref {
                current_fragment = unsafe { current_fragment_ref.next() };
                continue;
            }

            let current_fragment_id = get_id(current_fragment);
            let current_fragment_offset = (*offsets.get(&current_fragment_id).unwrap()) as isize;

            match current_fragment_ref {
                Fragment::Relaxable_Jump { variant, label } => {
                    if *variant == RelaxationVariant::Short {
                        let label_position = get_position(label, offsets)? as isize;
                        let diff = current_fragment_offset - label_position - const_sizes::SHORT_JUMP as isize;
                        if (diff < i8::MIN as isize - MAGIC_SHIFT) || (diff > i8::MAX as isize - MAGIC_SHIFT) {
                            *variant = RelaxationVariant::Long;
                            has_changes = true;
                            let add = const_sizes::LONG_JUMP - const_sizes::SHORT_JUMP;
                            update_subsequent_offsets!(current_fragment, add);
                        }
                    }
                }
                Fragment::Relaxable_CondJump { variant, label, .. } => {
                    if *variant == RelaxationVariant::Short {
                        let label_position = get_position(label, offsets)? as isize;
                        let diff = current_fragment_offset - label_position - const_sizes::SHORT_COND_JUMP as isize;
                        if (diff < i8::MIN as isize - MAGIC_SHIFT) || (diff > i8::MAX as isize - MAGIC_SHIFT) {
                            *variant = RelaxationVariant::Long;
                            has_changes = true;
                            let add = const_sizes::LONG_COND_JUMP - const_sizes::SHORT_COND_JUMP;
                            update_subsequent_offsets!(current_fragment, add);
                        }
                    }
                }
                Fragment::Bytes { .. } => unreachable!(),
            }

            current_fragment = unsafe { current_fragment_ref.next() };
        }

        if !has_changes {
            break;
        }
    }

    Ok(())
}

fn calculate_labels_map(
    asm: &X86_64Assembler,
    offsets: &HashMap<FragmentOrderId, u32>,
) -> Result<HashMap<Label, usize>, AssembleError> {
    let mut result = HashMap::with_capacity(asm.label_offsets.len());

    for (label, label_offset) in &asm.label_offsets {
        let fragment_index = label_offset.fragment_id.index();
        let fragment = fragment_at_index!(asm, fragment_index);
        let relaxation_offset = match fragment {
            Fragment::Bytes { .. } => 0,
            _ => fragment.data_length(),
        };

        let fragment_offset = offsets.get(&label_offset.fragment_id).unwrap();
        let position = *fragment_offset + relaxation_offset + label_offset.in_fragment_offset;
        result.insert(*label, position as usize);
    }

    Ok(result)
}

fn patch_addresses(
    asm: &mut X86_64Assembler,
    labels_map: &HashMap<Label, usize>,
    offsets: &HashMap<FragmentOrderId, u32>,
) -> Result<(), AssembleError> {
    unsafe {
        for (label, patchable_addresses) in &asm.patchable_addresses {
            let final_label_position = *labels_map.get(label).unwrap() as isize;
            for patchable_address in patchable_addresses.as_slice() {
                let patchable_fragment_id = patchable_address.instruction_position.fragment_id;
                let patchable_fragment_index = patchable_fragment_id.index();
                let patchable_fragment = fragment_at_index!(asm, patchable_fragment_index);
                debug_assert!(
                    matches!(patchable_fragment, Fragment::Bytes { .. }),
                    "Patchable fragment is not a bytes fragment. Got: {patchable_fragment:?}"
                );
                let patchable_fragment_data_offset = size_of::<Fragment>();
                let patchable_imm32_offset = patchable_fragment_index as usize
                    + patchable_fragment_data_offset
                    + patchable_address.instruction_position.in_fragment_offset as usize
                    + patchable_address.imm32_offset as usize;

                let patchable_imm32 = asm.fragments.as_mut_ptr().add(patchable_imm32_offset);

                let final_fragment_offset = *offsets.get(&patchable_fragment_id).unwrap() as isize;
                let final_end_of_instruction = final_fragment_offset
                    + patchable_address.instruction_length as isize
                    + patchable_address.instruction_position.in_fragment_offset as isize;
                let distance = final_label_position - final_end_of_instruction;
                debug_assert!(
                    distance >= i32::MIN as isize && distance <= i32::MAX as isize,
                    "Patchable distance is too far. Got: {distance}"
                );
                let distance = distance as i32;
                let imm32 = enc_models::Immediate32::from_i32(distance).encode();
                patchable_imm32.copy_from_nonoverlapping(imm32.as_ptr(), imm32.len());
            }
        }
    }

    Ok(())
}

fn emit_fragments(
    asm: &X86_64Assembler,
    labels_map: &HashMap<Label, usize>,
    offsets: &HashMap<FragmentOrderId, u32>,
    stream: &mut dyn std::io::Write,
) -> Result<EmissionData, AssembleError> {
    let start = fragment_at_index!(asm, 0) as *const Fragment;
    let end = fragment_end!(asm);

    let mut emitted_bytes = 0;
    let mut current = start;
    while current < end {
        let current_fragment_ref = unsafe { &*current };
        emitted_bytes += encode_fragment(asm, current_fragment_ref, labels_map, offsets, stream)?;
        current = unsafe { current_fragment_ref.next() };
    }

    let mut public_labels = HashMap::with_capacity(asm.public_labels.len());
    for item in &asm.public_labels {
        let position = labels_map.get(item).unwrap();
        public_labels.insert(*item, *position);
    }

    let emission_data = EmissionData::new(emitted_bytes, public_labels);
    Ok(emission_data)
}

fn encode_fragment(
    asm: &X86_64Assembler,
    fragment: &Fragment,
    labels_map: &HashMap<Label, usize>,
    offsets: &HashMap<FragmentOrderId, u32>,
    stream: &mut dyn std::io::Write,
) -> Result<usize, AssembleError> {
    let start = fragment_at_index!(asm, 0) as *const Fragment;
    let get_id = |fragment: *const Fragment| -> FragmentOrderId {
        let u8_fragment = fragment.cast::<u8>();
        let u8_start = start.cast::<u8>();
        let ptr_diff = unsafe { u8_fragment.offset_from(u8_start) };
        FragmentOrderId::from_index(ptr_diff as u32)
    };

    let get_fragment_position = |fragment: *const Fragment| {
        let id = get_id(fragment);
        *offsets.get(&id).unwrap() as isize
    };

    let emitted_bytes = match fragment {
        Fragment::Bytes { .. } => {
            let slice = unsafe {
                let self_ptr = std::ptr::from_ref(fragment).cast::<u8>();
                let data_ptr = self_ptr.add(size_of::<Fragment>());
                let len = fragment.data_length() as usize;
                std::slice::from_raw_parts(data_ptr, len)
            };
            stream.write_all(slice)?;
            slice.len()
        }
        Fragment::Relaxable_Jump { variant, label } => {
            let position = get_fragment_position(fragment);
            let label_position = (*labels_map.get(label).unwrap()) as isize;
            let diff = label_position - position;
            match variant {
                RelaxationVariant::Short => {
                    let diff = diff - const_sizes::SHORT_JUMP as isize;
                    debug_assert!(
                        diff >= i8::MIN as isize && diff <= i8::MAX as isize,
                        "Short relaxable jump is too far. Got: {diff}"
                    );
                    let imm8 = enc_models::Immediate8::from_i8(diff as i8);
                    let encoded = enc::jmp::encode_jmp_imm8(imm8);
                    stream.write_all(encoded.as_slice())?;
                    const_sizes::SHORT_JUMP as usize
                }
                RelaxationVariant::Long => {
                    let diff = diff - const_sizes::LONG_JUMP as isize;
                    debug_assert!(
                        diff >= i32::MIN as isize && diff <= i32::MAX as isize,
                        "Long relaxable jump is too far. Got: {diff}"
                    );
                    let imm32 = enc_models::Immediate32::from_i32(diff as i32);
                    let encoded = enc::jmp::encode_jmp_imm32(imm32);
                    stream.write_all(encoded.as_slice())?;
                    const_sizes::LONG_JUMP as usize
                }
            }
        }
        Fragment::Relaxable_CondJump {
            variant,
            condition,
            label,
        } => {
            let position = get_fragment_position(fragment);
            let label_position = (*labels_map.get(label).unwrap()) as isize;
            let diff = label_position - position;
            match variant {
                RelaxationVariant::Short => {
                    let diff = diff - const_sizes::SHORT_COND_JUMP as isize;
                    debug_assert!(
                        diff >= i8::MIN as isize && diff <= i8::MAX as isize,
                        "Short relaxable jump is too far."
                    );
                    let imm8 = enc_models::Immediate8::from_i8(diff as i8);
                    let encoded = encode_short_cond_jump(*condition, imm8);
                    stream.write_all(encoded.as_slice())?;
                    const_sizes::SHORT_COND_JUMP as usize
                }
                RelaxationVariant::Long => {
                    let diff = diff - const_sizes::LONG_COND_JUMP as isize;
                    debug_assert!(
                        diff >= i32::MIN as isize && diff <= i32::MAX as isize,
                        "Long relaxable jump is too far."
                    );
                    let imm32 = enc_models::Immediate32::from_i32(diff as i32);
                    let encoded = encode_long_cond_jump(*condition, imm32);
                    stream.write_all(encoded.as_slice())?;
                    const_sizes::LONG_COND_JUMP as usize
                }
            }
        }
    };

    Ok(emitted_bytes)
}

fn encode_short_cond_jump(cond: Condition, imm8: enc_models::Immediate8) -> enc_models::EncodedX86_64Instruction {
    match cond {
        Condition::Equal => enc::jcc::encode_jcc_E_imm8(imm8),
        Condition::NotEqual => enc::jcc::encode_jcc_NE_imm8(imm8),
        Condition::Above => enc::jcc::encode_jcc_A_imm8(imm8),
        Condition::AboveOrEqual => enc::jcc::encode_jcc_AE_imm8(imm8),
        Condition::Below => enc::jcc::encode_jcc_B_imm8(imm8),
        Condition::BelowOrEqual => enc::jcc::encode_jcc_BE_imm8(imm8),
        Condition::Greater => enc::jcc::encode_jcc_G_imm8(imm8),
        Condition::GreaterOrEqual => enc::jcc::encode_jcc_GE_imm8(imm8),
        Condition::Less => enc::jcc::encode_jcc_L_imm8(imm8),
        Condition::LessOrEqual => enc::jcc::encode_jcc_LE_imm8(imm8),
        Condition::Overflow => enc::jcc::encode_jcc_O_imm8(imm8),
        Condition::NotOverflow => enc::jcc::encode_jcc_NO_imm8(imm8),
        Condition::Parity => enc::jcc::encode_jcc_P_imm8(imm8),
        Condition::NotParity => enc::jcc::encode_jcc_NP_imm8(imm8),
        Condition::ParityOdd => enc::jcc::encode_jcc_PO_imm8(imm8),
        Condition::ParityEven => enc::jcc::encode_jcc_PE_imm8(imm8),
        Condition::Sign => enc::jcc::encode_jcc_S_imm8(imm8),
        Condition::NotSign => enc::jcc::encode_jcc_NS_imm8(imm8),
        Condition::Carry => enc::jcc::encode_jcc_C_imm8(imm8),
        Condition::NotCarry => enc::jcc::encode_jcc_NC_imm8(imm8),
    }
}

fn encode_long_cond_jump(cond: Condition, imm32: enc_models::Immediate32) -> enc_models::EncodedX86_64Instruction {
    match cond {
        Condition::Equal => enc::jcc::encode_jcc_E_imm32(imm32),
        Condition::NotEqual => enc::jcc::encode_jcc_NE_imm32(imm32),
        Condition::Above => enc::jcc::encode_jcc_A_imm32(imm32),
        Condition::AboveOrEqual => enc::jcc::encode_jcc_AE_imm32(imm32),
        Condition::Below => enc::jcc::encode_jcc_B_imm32(imm32),
        Condition::BelowOrEqual => enc::jcc::encode_jcc_BE_imm32(imm32),
        Condition::Greater => enc::jcc::encode_jcc_G_imm32(imm32),
        Condition::GreaterOrEqual => enc::jcc::encode_jcc_GE_imm32(imm32),
        Condition::Less => enc::jcc::encode_jcc_L_imm32(imm32),
        Condition::LessOrEqual => enc::jcc::encode_jcc_LE_imm32(imm32),
        Condition::Overflow => enc::jcc::encode_jcc_O_imm32(imm32),
        Condition::NotOverflow => enc::jcc::encode_jcc_NO_imm32(imm32),
        Condition::Parity => enc::jcc::encode_jcc_P_imm32(imm32),
        Condition::NotParity => enc::jcc::encode_jcc_NP_imm32(imm32),
        Condition::ParityOdd => enc::jcc::encode_jcc_PO_imm32(imm32),
        Condition::ParityEven => enc::jcc::encode_jcc_PE_imm32(imm32),
        Condition::Sign => enc::jcc::encode_jcc_S_imm32(imm32),
        Condition::NotSign => enc::jcc::encode_jcc_NS_imm32(imm32),
        Condition::Carry => enc::jcc::encode_jcc_C_imm32(imm32),
        Condition::NotCarry => enc::jcc::encode_jcc_NC_imm32(imm32),
    }
}

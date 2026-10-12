use super::{helpers::function_matches_entry_point, Error, Writer};
use crate::valid::ModuleInfo;
use crate::{AddressSpace, Block, Expression, Function, ImageClass, Module, Statement, TypeInner};
use alloc::vec;

impl Writer {
    pub(super) fn prepare_combined_image_samplers(
        &mut self,
        module: &Module,
        info: &ModuleInfo,
        entry_point: Option<usize>,
    ) -> Result<(), Error> {
        if self.combined_image_sampler_bindings.is_empty() {
            return Ok(());
        }
        let mut image_bindings = crate::FastHashSet::default();
        let mut sampler_bindings = crate::FastHashSet::default();
        for &(image, sampler) in &self.combined_image_sampler_bindings {
            if image == sampler
                || !image_bindings.insert(image)
                || !sampler_bindings.insert(sampler)
            {
                return Err(Error::Validation(
                    "duplicate combined image/sampler binding",
                ));
            }
        }
        let entry_info = entry_point.map(|index| info.get_entry_point(index));
        let mut globals_by_binding = crate::FastHashMap::default();
        for (handle, global) in module.global_variables.iter() {
            if entry_info.is_some_and(|entry| entry[handle].is_empty()) {
                continue;
            }
            if let Some(binding) = global.binding {
                if (image_bindings.contains(&binding) || sampler_bindings.contains(&binding))
                    && globals_by_binding.insert(binding, handle).is_some()
                {
                    return Err(Error::Validation(
                        "ambiguous combined image/sampler binding",
                    ));
                }
            }
        }
        for &(image_binding, sampler_binding) in &self.combined_image_sampler_bindings {
            let image = globals_by_binding.get(&image_binding).copied();
            let sampler = globals_by_binding.get(&sampler_binding).copied();
            let image = match (image, sampler) {
                (None, None) => continue,
                (None, Some(_)) => return Err(Error::Validation("combined sampler has no image")),
                (Some(image), _) => image,
            };
            let var = &module.global_variables[image];
            if var.space != AddressSpace::Handle
                || !matches!(
                    module.types[var.ty].inner,
                    TypeInner::Image {
                        class: ImageClass::Sampled { multi: false, .. },
                        ..
                    }
                )
            {
                return Err(Error::Validation("invalid combined image"));
            }
            if let Some(sampler) = sampler {
                let var = &module.global_variables[sampler];
                if var.space != AddressSpace::Handle
                    || !matches!(
                        module.types[var.ty].inner,
                        TypeInner::Sampler { comparison: false }
                    )
                {
                    return Err(Error::Validation("invalid combined sampler"));
                }
                self.combined_sampler_images.insert(sampler, image);
            }
            self.combined_image_samplers.insert(image, sampler);
        }
        for (handle, function) in module.functions.iter() {
            if entry_info.is_some_and(|entry| !function_matches_entry_point(&info[handle], entry)) {
                continue;
            }
            self.validate_combined_sampler_uses(function)?;
        }
        for (index, entry) in module.entry_points.iter().enumerate() {
            if entry_point.is_none() || entry_point == Some(index) {
                self.validate_combined_sampler_uses(&entry.function)?;
            }
        }
        Ok(())
    }

    fn validate_combined_sampler_uses(&self, function: &Function) -> Result<(), Error> {
        for (_, expression) in function.expressions.iter() {
            let Expression::ImageSample { image, sampler, .. } = *expression else {
                continue;
            };
            let Expression::GlobalVariable(sampler) = function.expressions[sampler] else {
                continue;
            };
            let Some(&expected_image) = self.combined_sampler_images.get(&sampler) else {
                continue;
            };
            if !matches!(function.expressions[image], Expression::GlobalVariable(image) if image == expected_image)
            {
                return Err(Error::Validation(
                    "combined sampler used with a different image",
                ));
            }
        }
        let mut blocks: vec::Vec<&Block> = vec![&function.body];
        while let Some(block) = blocks.pop() {
            for statement in block.iter() {
                match *statement {
                    Statement::Call { ref arguments, .. } => {
                        for &argument in arguments {
                            let Expression::GlobalVariable(global) = function.expressions[argument]
                            else {
                                continue;
                            };
                            if self.combined_image_samplers.contains_key(&global)
                                || self.combined_sampler_images.contains_key(&global)
                            {
                                return Err(Error::Validation(
                                    "combined resources cannot be passed to functions",
                                ));
                            }
                        }
                    }
                    Statement::Block(ref block) => blocks.push(block),
                    Statement::If {
                        ref accept,
                        ref reject,
                        ..
                    } => {
                        blocks.push(accept);
                        blocks.push(reject);
                    }
                    Statement::Switch { ref cases, .. } => {
                        blocks.extend(cases.iter().map(|case| &case.body))
                    }
                    Statement::Loop {
                        ref body,
                        ref continuing,
                        ..
                    } => {
                        blocks.push(body);
                        blocks.push(continuing);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

use graphql::scalars;
use graphql::scalars::unprocessed;
use graphql::scheme::Unprocessed;
use std::collections::HashMap;

pub struct Scheme {
    pub query: String,
    pub mutate: Option<String>,
}

pub type GsdlScalarMap = HashMap<String, unprocessed::GsdlScalar>;

// Ideally, we want all type references to be stored as exactly that, Rust references.
// However, current Rust borrow rules do not permit self-referential structs.
// See https://internals.rust-lang.org/t/improving-self-referential-structs/4808 for discussion.
// Therefore, unprocessed types are stored inside the processed scheme and internal HashMap is used
// to map names to actual scalars.
// The implementation promises that type structure is consistent.
// Whether it manages to keep its promise is another matter entirely
pub struct Processed {
    // internal structure holding all used scalars
    scalar_map: GsdlScalarMap,
    enums: Vec<String>,
    interfaces: Vec<String>,
    scheme: Scheme,
    types: Vec<String>,
    unions: Vec<String>,
}

impl Processed {
    pub fn enums(&self) -> scalars::processed::Enums {
        scalars::processed::Enums::from(self.enums.iter(), &self.scalar_map)
    }

    pub fn interfaces(&self) -> scalars::processed::Interfaces {
        scalars::processed::Interfaces::from(self.interfaces.iter(), &self.scalar_map)
    }

    pub fn types(&self) -> scalars::processed::Types {
        scalars::processed::Types::from(self.types.iter(), &self.scalar_map)
    }

    pub fn unions(&self) -> scalars::processed::Unions {
        scalars::processed::Unions::from(self.unions.iter(), &self.scalar_map)
    }

    pub fn mutate(&self) -> Option<scalars::processed::Type> {
        if let Some(ref mutate) = self.scheme.mutate {
            Some(scalars::processed::Type::from(mutate, &self.scalar_map))
        } else {
            None
        }
    }

    pub fn query(&self) -> scalars::processed::Type {
        scalars::processed::Type::from(&self.scheme.query, &self.scalar_map)
    }

    pub fn from(unprocessed: Unprocessed) -> Result<Processed, Vec<String>> {
        let result = Processed::build(unprocessed)?;
        let mut errors = vec![];

        // verify internal consistency
        // assert are failed and panic are triggered for our bugs

        // step 1: check types (as opposed to interfaces, enums and unions)
        for type_name in &result.types {
            let gsdl_type = result.scalar_map.get(type_name).expect(&format!(
                "Cannot find type {} in internal scalar map",
                type_name
            ));
            match *gsdl_type {
                unprocessed::GsdlScalar::Type(ref gsdl_type) => {
                    assert_eq!(*type_name, gsdl_type.name);

                    // step 1.1: check referenced interfaces
                    for interface_name in &gsdl_type.implements {
                        let interface = result.scalar_map.get(interface_name);
                        match interface {
                            Some(interface) => match *interface {
                                unprocessed::GsdlScalar::Interface(ref interface) => {
                                    assert_eq!(*interface_name, interface.name);
                                    for interface_field in &interface.fields {
                                        // using binary_search_by because binary_search_by_key
                                        // requires key derivation function to return B, not &B
                                        match gsdl_type
                                            .fields
                                            .binary_search_by(|x| x.name.cmp(&interface_field.name))
                                        {
                                            Ok(i) => {
                                                let type_field = &gsdl_type.fields[i];
                                                assert_eq!(interface_field.name, type_field.name);
                                                if type_field != interface_field {
                                                    errors.push(format!(
                                                        "Type {} implements {}, \
                                                         but field named {} is different: \
                                                         {:?} in type, \
                                                         {:?} in interface",
                                                        gsdl_type.name,
                                                        interface_name,
                                                        interface_field.name,
                                                        type_field,
                                                        interface_field
                                                    ))
                                                }
                                            }
                                            Err(_) => errors.push(format!(
                                                "Type {} implements {}, \
                                                 but does not contain field named {}",
                                                gsdl_type.name,
                                                interface_name,
                                                &interface_field.name
                                            )),
                                        }
                                    }
                                }
                                _ => errors.push(format!(
                                    "Type {} implements {}, but {} is not interface, \
                                     but {:?} instead",
                                    gsdl_type.name, interface_name, interface_name, interface
                                )),
                            },
                            None => errors.push(format!(
                                "Type {} implements {}, \
                                 but {} is not defined",
                                gsdl_type.name, interface_name, interface_name
                            )),
                        };
                    }

                    // step 1.2: check that fields reference known scalars
                    for field in &gsdl_type.fields {
                        errors
                            .append(&mut result.check_field(field, &format!("Type {}", gsdl_type.name)));
                    }
                }
                _ => panic!(format!(
                    "Type {} is not type but {:?} in internal scalar map",
                    type_name, gsdl_type
                )),
            }
        }

        // step 2: check interfaces
        for interface_name in &result.interfaces {
            let interface = result.scalar_map.get(interface_name).expect(&format!(
                "Cannot find interface {} in internal scalar map",
                interface_name
            ));
            match *interface {
                unprocessed::GsdlScalar::Interface(ref interface) => {
                    assert_eq!(*interface_name, interface.name);

                    // check that fields reference known scalars
                    for field in &interface.fields {
                        errors.append(&mut result
                            .check_field(field, &format!("Interface {}", interface.name)));
                    }
                }
                _ => panic!(format!(
                    "Interface {} is not interface but {:?} in internal scalar map",
                    interface_name, interface
                )),
            }
        }

        // step 3: check scheme entry points
        // step 3.1: check query entry point
        match result.scalar_map.get(&result.scheme.query) {
            Some(gsdl_type) => match *gsdl_type {
                unprocessed::GsdlScalar::Type(ref gsdl_type) => {
                    assert_eq!(result.scheme.query, gsdl_type.name)
                }
                _ => errors.push(format!(
                    "Scheme query entry point {} is not a type but {:?} instead",
                    result.scheme.query, gsdl_type
                )),
            },
            None => errors.push(format!(
                "Scheme query entry point {} type is not defined",
                result.scheme.query
            )),
        }
        // step 3.2: check mutate entry point
        if let Some(ref mutate) = result.scheme.mutate {
            match result.scalar_map.get(mutate) {
                Some(gsdl_type) => match *gsdl_type {
                    unprocessed::GsdlScalar::Type(ref gsdl_type) => {
                        assert_eq!(result.scheme.query, gsdl_type.name)
                    }
                    _ => errors.push(format!(
                        "Scheme mutate entry point {} is not a type but {:?} instead",
                        result.scheme.query, gsdl_type
                    )),
                },
                None => errors.push(format!(
                    "Scheme mutate entry point {} type is not defined",
                    result.scheme.query
                )),
            }
        }

        if errors.is_empty() {
            Ok(result)
        } else {
            Err(errors)
        }
    }

    // check that field references known scalars
    fn check_field(&self, field: &unprocessed::Field, parent: &str) -> Vec<String> {
        let mut errors = vec![];

        // step 1: Check field return type
        let gsdl_field_type = self.scalar_map.get(&field.field_type.inner.name);
        match gsdl_field_type {
            Some(gsdl_scalar) => assert_eq!(*field.field_type.inner.name, *gsdl_scalar.name()),
            None => errors.push(format!(
                "{} field {} uses scalar {}, but {} is not defined",
                parent, field.name, field.field_type.inner.name, field.field_type.inner.name
            )),
        };

        // step 2: Check field arguments
        for argument in &field.arguments {
            let gsdl_argument_type = self.scalar_map.get(&argument.argument_type.inner.name);
            match gsdl_argument_type {
                Some(gsdl_scalar) => {
                    assert_eq!(*argument.argument_type.inner.name, *gsdl_scalar.name())
                }
                None => errors.push(format!(
                    "{} field {}  argument {} references type {}, but {} is not defined",
                    parent,
                    field.name,
                    argument.name,
                    argument.argument_type.inner.name,
                    argument.argument_type.inner.name
                )),
            };
        }

        errors
    }

    // internal function to build Processed with minimal checks
    fn build(unprocessed: Unprocessed) -> Result<Processed, Vec<String>> {
        // step 1
        // init internal structure holding all possible scalars
        let mut scalar_map = HashMap::with_capacity(
            unprocessed::GsdlBuiltinScalar::count() + unprocessed.enums.len()
                + unprocessed.interfaces.len() + unprocessed.types.len()
                + unprocessed.unions.len(),
        );

        // step 2: add builtin scalars
        for &(ref builtin_type, builtin_type_name) in unprocessed::GsdlBuiltinScalar::iter() {
            match scalar_map.insert(
                String::from(builtin_type_name),
                unprocessed::GsdlScalar::Builtin(*builtin_type),
            ) {
                None => (),
                Some(gsdl_type) => {
                    panic!(format!("Builtin type {:?}  is defined twice", gsdl_type))
                }
            }
        }

        let mut errors = vec![];

        // step 3: add enums
        let mut enums = Vec::with_capacity(unprocessed.enums.len());
        for gsdl_enum in unprocessed.enums {
            let enum_name = gsdl_enum.name.to_owned();
            let key = gsdl_enum.name.to_owned();
            match scalar_map.insert(key, unprocessed::GsdlScalar::Enum(gsdl_enum)) {
                None => (),
                Some(gsdl_type) => errors.push(format!(
                    "Enum {} is already defined as {:?}",
                    enum_name, gsdl_type
                )),
            }
            enums.push(enum_name);
        }
        enums.sort_unstable();

        // step 4: add interfaces
        let mut interfaces = Vec::with_capacity(unprocessed.interfaces.len());
        for interface in unprocessed.interfaces {
            let interface_name = interface.name.to_owned();
            let key = interface.name.to_owned();
            match scalar_map.insert(key, unprocessed::GsdlScalar::Interface(interface)) {
                None => (),
                Some(gsdl_type) => errors.push(format!(
                    "Interface {} is already defined as {:?}",
                    interface_name, gsdl_type
                )),
            }
            interfaces.push(interface_name);
        }
        interfaces.sort_unstable();

        // step 5: add types
        let mut types = Vec::with_capacity(unprocessed.types.len());
        for gsdl_type in unprocessed.types {
            let type_name = gsdl_type.name.to_owned();
            let key = gsdl_type.name.to_owned();
            match scalar_map.insert(key, unprocessed::GsdlScalar::Type(gsdl_type)) {
                None => (),
                Some(gsdl_type) => errors.push(format!(
                    "Type {} is already defined as {:?}",
                    type_name, gsdl_type
                )),
            };
            types.push(type_name);
        }
        types.sort_unstable();

        // step 6: add unions
        let mut unions = vec![];
        for union in unprocessed.unions {
            let union_name = union.name.to_owned();
            let key = union.name.to_owned();
            match scalar_map.insert(key, unprocessed::GsdlScalar::Union(union)) {
                None => (),
                Some(gsdl_type) => errors.push(format!(
                    "Union {} is already defined as {:?}",
                    union_name, gsdl_type
                )),
            };
            unions.push(union_name);
        }
        unions.sort_unstable();

        // step 7: init scheme entry points
        let scheme = Scheme {
            query: match unprocessed.query {
                Some(query) => query,
                None => {
                    errors.push(String::from("No scheme query entry point defined"));
                    String::from("")
                }
            },
            mutate: unprocessed.mutate,
        };

        if errors.is_empty() {
            Ok(Processed {
                scalar_map,
                enums,
                interfaces,
                scheme,
                types,
                unions,
            })
        } else {
            Err(errors)
        }
    }
}

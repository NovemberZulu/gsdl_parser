use graphql::data::unprocessed::{Enum, Field, GsdlItem, Interface, SchemeEntryPoints, Type, Union};
use graphql::scheme::Processed;

pub struct Unprocessed {
    pub enums: Vec<Enum>,
    pub interfaces: Vec<Interface>,

    // Scheme entry points
    pub query: Option<String>,
    pub mutate: Option<String>,
    pub scheme_entry_points_encountered: bool,

    pub types: Vec<Type>,
    pub unions: Vec<Union>,
}

impl Unprocessed {
    fn from(items: Vec<GsdlItem>) -> Result<Unprocessed, Vec<String>> {
        let mut result = Unprocessed {
            enums: vec![],
            interfaces: vec![],
            query: None,
            mutate: None,
            scheme_entry_points_encountered: false,
            types: vec![],
            unions: vec![],
        };

        items.into_iter().map(|item| result.add_item(item)).fold(
            Ok(()),
            |a: Result<(), Vec<String>>, b| {
                b.map_err(|c| match a {
                    Ok(()) => c,
                    Err(errors) => {
                        let mut new_errors = errors.to_owned();
                        new_errors.extend(c);
                        new_errors
                    }
                })
            },
        )?;

        Ok(result)
    }

    pub fn process(self) -> Result<Processed, Vec<String>> {
        Processed::from(self)
    }

    fn add_item(&mut self, item: GsdlItem) -> Result<(), Vec<String>> {
        match item {
            GsdlItem::Enum(gsdl_enum) => self.add_enum(gsdl_enum),
            GsdlItem::Interface(interface) => self.add_interface(interface),
            GsdlItem::SchemeEntryPoints(scheme_entry_points) => {
                self.add_scheme_entry_points(scheme_entry_points)
            }
            GsdlItem::Type(gsdl_type) => self.add_type(gsdl_type),
            GsdlItem::Union(union) => self.add_union(union),
        }
    }

    fn add_enum(&mut self, gsdl_enum: Enum) -> Result<(), Vec<String>> {
        let mut sorted_values = gsdl_enum.values;
        sorted_values.sort_unstable();

        let mut errors = vec![];
        // check value uniqueness
        {
            let mut iter = sorted_values.iter().peekable();
            while let Some(value) = iter.next() {
                if let Some(&next_value) = iter.peek() {
                    if *next_value == *value {
                        errors.push(format!(
                            "Enum {} has duplicate value {}",
                            gsdl_enum.name, value
                        ))
                    }
                }
            }
        }

        self.enums.push(Enum {
            name: gsdl_enum.name,
            values: sorted_values,
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn add_interface(&mut self, interface: Interface) -> Result<(), Vec<String>> {
        let mut sorted_fields = interface.fields;
        // Note using sort_unstable_by instead of sort_unstable_by_key due to latter being more restrictive than former
        // See https://github.com/rust-lang/rust/issues/34162 for details
        sorted_fields.sort_unstable_by(|l, r| l.name.cmp(&r.name));

        let mut errors = vec![];
        // check field name uniqueness
        {
            let mut iter = sorted_fields.iter().peekable();
            while let Some(field) = iter.next() {
                if let Some(&next_field) = iter.peek() {
                    if *next_field.name == *field.name {
                        errors.push(format!(
                            "Interface {} has duplicate field named {}",
                            interface.name, field.name
                        ))
                    }
                }
            }
        }
        // check argument name uniqueness per field
        let prefix = format!("Interface {}", interface.name);
        let processed_fields: Vec<Field> = sorted_fields
            .into_iter()
            .map(|f| Unprocessed::process_field(f, &mut errors, &prefix))
            .collect();

        self.interfaces.push(Interface {
            name: interface.name,
            fields: processed_fields,
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn add_scheme_entry_points(
        &mut self,
        scheme_entry_points: SchemeEntryPoints,
    ) -> Result<(), Vec<String>> {
        let mut errors = vec![];

        if self.scheme_entry_points_encountered {
            errors.push(format!(
                "Duplicate schema entry points : already set query: {:?}, mutate {:?}, new  {:?}",
                self.query, self.mutate, scheme_entry_points
            ))
        }
        self.scheme_entry_points_encountered = true;

        if scheme_entry_points.entries.is_empty() {
            errors.push(String::from("Empty schema entry points  encountered"))
        }

        for (name, value) in scheme_entry_points.entries {
            match name.as_ref() {
                "query" => {
                    match self.query {
                        None => (),
                        Some(ref old_value) => errors.push(format!(
                            "Duplicate query entry in scheme: old {}, new {}",
                            old_value, value
                        )),
                    };
                    self.query = Some(value)
                }
                "mutate" => {
                    match self.mutate {
                        None => (),
                        Some(ref old_value) => errors.push(format!(
                            "Duplicate mutate entry in scheme: old {}, new {}",
                            old_value, value
                        )),
                    };
                    ;
                    self.mutate = Some(value)
                }
                _ => errors.push(format!("Unknown entry {} in scheme", name)),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn add_type(&mut self, gsdl_type: Type) -> Result<(), Vec<String>> {
        let mut sorted_implements = gsdl_type.implements;
        sorted_implements.sort_unstable();

        let mut sorted_fields = gsdl_type.fields;
        // Note using sort_unstable_by instead of sort_unstable_by_key due to latter being more restrictive than former
        // See https://github.com/rust-lang/rust/issues/34162 for details
        sorted_fields.sort_unstable_by(|l, r| l.name.cmp(&r.name));

        let mut errors = vec![];
        {
            // check interface name uniqueness
            let mut iter = sorted_implements.iter().peekable();
            while let Some(interface) = iter.next() {
                if let Some(&next_interface) = iter.peek() {
                    if *next_interface == *interface {
                        errors.push(format!(
                            "Type {} implements named {} twice",
                            gsdl_type.name, interface
                        ))
                    }
                }
            }
            //check filed name uniquness per field
            let mut iter = sorted_fields.iter().peekable();
            while let Some(field) = iter.next() {
                if let Some(&next_field) = iter.peek() {
                    if *next_field.name == *field.name {
                        errors.push(format!(
                            "Type {} has duplicate field named {}",
                            gsdl_type.name, field.name
                        ))
                    }
                }
            }
        }

        let prefix = format!("Type {}", gsdl_type.name);
        let processed_fields: Vec<Field> = sorted_fields
            .into_iter()
            .map(|f| Unprocessed::process_field(f, &mut errors, &prefix))
            .collect();

        self.types.push(Type {
            name: gsdl_type.name,
            implements: sorted_implements,
            fields: processed_fields,
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn add_union(&mut self, union: Union) -> Result<(), Vec<String>> {
        let mut sorted_members = union.members;
        sorted_members.sort_unstable();

        let mut errors = vec![];
        // check member uniqueness
        {
            let mut iter = sorted_members.iter().peekable();
            while let Some(member) = iter.next() {
                if let Some(&next_member) = iter.peek() {
                    if *next_member == *member {
                        errors.push(format!(
                            "Union {} has duplicate member {}",
                            union.name, member
                        ))
                    }
                }
            }
        }

        self.unions.push(Union {
            name: union.name,
            members: sorted_members,
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn process_field(field: Field, errors: &mut Vec<String>, prefix: &str) -> Field {
        let mut result = field;

        // Note using sort_unstable_by instead of sort_unstable_by_key due to latter being more restrictive than former
        // See https://github.com/rust-lang/rust/issues/34162 for details
        result
            .arguments
            .sort_unstable_by(|l, r| l.name.cmp(&r.name));

        {
            let mut iter = result.arguments.iter().peekable();
            while let Some(argument) = iter.next() {
                if let Some(&next_argument) = iter.peek() {
                    if *next_argument.name == *argument.name {
                        errors.push(format!(
                            "{} field {} has duplicate argument named {}",
                            prefix, result.name, argument.name
                        ))
                    }
                }
            }
        }

        result
    }
}

pub trait UnprocessedSource {
    fn build_gsdl(self) -> Result<Unprocessed, Vec<String>>;
}

impl UnprocessedSource for Vec<GsdlItem> {
    fn build_gsdl(self) -> Result<Unprocessed, Vec<String>> {
        Unprocessed::from(self)
    }
}

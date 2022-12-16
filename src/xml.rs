use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Error};
use log::{debug, error, info};
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

use crate::types::OperationResult;

#[derive(Debug,PartialEq,Clone)]
pub struct XmlConfig {
    pub categories: Vec<MetaProperty>,
    pub clients: Vec<MetaProperty>,
    pub accounts: Vec<Account>
}

#[derive(Debug,PartialEq,Clone)]
pub struct MetaProperty {
    pub id: u16,
    pub name: String,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Account {
    pub id: u16,
    pub name: String,
    pub client_id: u16,
    pub category_id: u16,
    pub login: String
}

const UNINITIALIZED_ID_VALUE: u16 = 10000;

/// - Name
/// - Category Name
/// - Client Name
/// - Login
pub fn get_xml_config_from_file(file_path: &Path) -> OperationResult<XmlConfig> {
    info!("load xml configuration from file '{}'", file_path.display());
    let xml = fs::read_to_string(file_path)
                            .context("couldn't read xml file")?;

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut categories: Vec<MetaProperty> = vec![];
    let mut clients: Vec<MetaProperty> = vec![];
    let mut accounts: Vec<Account> = vec![];

    let mut current_id: u16 = UNINITIALIZED_ID_VALUE;
    let mut current_name = String::new();
    let mut current_login = String::new();
    let mut current_client_id: u16 = UNINITIALIZED_ID_VALUE;
    let mut current_category_id: u16 = UNINITIALIZED_ID_VALUE;

    loop {

        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                debug!("open tag '{:?}'", e.name().as_ref());

                match e.name().as_ref() {
                    b"Category" | b"Client" | b"Account" => {
                        let mut attrs = e.attributes();

                        let id_value = get_element_id_attribute(&mut attrs)?;

                        match id_value {
                            Some(id) => {
                                current_id = id;
                                info!("id - {}", id);
                            }
                            None => error!("tag doesn't have 'id' attribute")
                        }
                    },
                    b"name" => {
                        let txt = reader
                            .read_text(e.name())
                            .expect("cannot decode text value");
                        debug!("name: {:?}", txt);

                        current_name = txt.to_string();
                    },
                    b"login" => {
                        let txt = reader
                            .read_text(e.name())
                            .expect("cannot decode text value");
                        debug!("name: {:?}", txt);

                        current_login = txt.to_string();
                    },
                    b"clientId" => {
                        let txt = reader
                            .read_text(e.name())
                            .expect("cannot decode text value");
                        debug!("name: {:?}", txt);

                        let value = txt.to_string();

                        current_client_id = value.parse::<u16>()?;
                    },
                    b"categoryId" => {
                        let txt = reader
                            .read_text(e.name())
                            .expect("cannot decode text value");
                        debug!("name: {:?}", txt);

                        let value = txt.to_string();

                        current_category_id = value.parse::<u16>()?;
                    },
                    _ => (),
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"Category" => {
                        info!("close category tag");
                        categories.push(
                            MetaProperty {
                                id: current_id,
                                name: current_name.to_string(),
                            }
                        );

                        current_id = UNINITIALIZED_ID_VALUE;
                    },
                    b"Client" => {
                        info!("close client tag");
                        clients.push(
                            MetaProperty {
                                id: current_id,
                                name: current_name.to_string(),
                            }
                        );

                        current_id = UNINITIALIZED_ID_VALUE;
                        current_name = String::new();
                    },
                    b"Account" => {
                        info!("close client tag");
                        accounts.push(
                            Account {
                                id: current_id,
                                name: current_name.to_string(),
                                client_id: current_client_id,
                                category_id: current_category_id,
                                login: current_login.to_string(),
                            }
                        );

                        current_id = UNINITIALIZED_ID_VALUE;
                        current_name = String::new();
                        current_client_id = UNINITIALIZED_ID_VALUE;
                        current_category_id = UNINITIALIZED_ID_VALUE;
                        current_login = String::new();
                    },
                    _ => {}
                }

            }
            _ => (),
        }
        buf.clear();
    }

    let config = XmlConfig {
        categories: categories.clone(),
        clients: clients.clone(),
        accounts: accounts.clone(),
    };

    debug!("---[xml config]---");
    debug!("{:?}", config);
    debug!("---[/xml config]---");

    Ok(config)
}

fn get_element_id_attribute(attrs: &mut Attributes) -> Result<Option<u16>, Error> {
    let attribute = attrs.find(|a|{
        match a {
            Ok(au) => {
                au.key == QName(b"id")
            }
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    });

    match attribute {
        Some(attribute_value) => {

            match attribute_value {
                Ok(value) => {
                    let id_value = String::from_utf8(value.value.to_vec())?;

                    let id = id_value.parse::<u16>()?;

                    Ok(Some(id))
                }
                Err(e) => {
                    error!("{}", e);
                    Err(anyhow!("couldn't read id attribute value"))
                }
            }

        }
        None => {
            Ok(None)
        }
    }

}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::tests::init_logging;
    use crate::xml::{Account, get_xml_config_from_file, MetaProperty, XmlConfig};

    #[test]
    fn config_should_be_loaded() {
        init_logging();

        let expected_xml_config = XmlConfig {
            categories: vec![
                MetaProperty {
                    id: 1,
                    name: "APP".to_string(),
                }
            ],
            clients: vec![
                MetaProperty {
                    id: 1,
                    name: "BirchStore".to_string(),
                }
            ],
            accounts: vec![
                Account {
                    id: 1,
                    name: "Ivan Petrov".to_string(),
                    client_id: 1,
                    category_id: 1,
                    login: "i.petrov".to_string(),
                },
                Account {
                    id: 2,
                    name: "Abramova Nina".to_string(),
                    client_id: 1,
                    category_id: 1,
                    login: "n.abramova".to_string(),
                }
            ],
        };

        let xml_file_path = Path::new("test-data").join("import.xml");

        match get_xml_config_from_file(xml_file_path.as_path()) {
            Ok(xml_config) => {
                assert_eq!(xml_config, expected_xml_config);
            }
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("{}", e.root_cause());
                panic!("result expected")
            }
        }
    }
}

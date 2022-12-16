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

/// Extract from given xml file partial properties for entities:
/// - category
/// - client
/// - account
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

    let mut syntax_error = false;

    loop {

        match reader.read_event_into(&mut buf) {
            Err(e) => {
                error!("xml has syntax error(s): {}", e);
                syntax_error = true;
                break;
            },
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec())?;
                debug!("open tag '{}'", tag_name);

                match e.name().as_ref() {
                    b"Category" | b"Client" | b"Account" => {
                        let mut attrs = e.attributes();

                        let id_value = get_element_id_attribute(&mut attrs)?;

                        match id_value {
                            Some(id) => {
                                current_id = id;
                                debug!("id: {}", id);
                            }
                            None => error!("tag doesn't have 'id' attribute")
                        }
                    },
                    b"name" => {
                        let value = get_element_text(&mut reader, e.name())?;
                        debug!("name: {}", value);
                        current_name = value;
                    },
                    b"login" => {
                        let value = get_element_text(&mut reader, e.name())?;
                        debug!("login: {}", value);
                        current_login = value;
                    },
                    b"clientId" => {
                        let value = get_element_text(&mut reader, e.name())?;
                        debug!("client id: {}", value);
                        current_client_id = value.parse::<u16>()?;
                    },
                    b"categoryId" => {
                        let value = get_element_text(&mut reader, e.name())?;
                        debug!("category id: {}", value);
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

    if !syntax_error {
        let config = XmlConfig {
            categories: categories.clone(),
            clients: clients.clone(),
            accounts: accounts.clone(),
        };

        debug!("---[xml config]---");
        debug!("{:?}", config);
        debug!("---[/xml config]---");

        Ok(config)

    } else {
        Err(anyhow!("xml parse error(s)"))
    }
}

fn get_element_id_attribute(attrs: &mut Attributes) -> Result<Option<u16>, Error> {
    let attribute = attrs.find(|a|{
        match a {
            Ok(value) => value.key == QName(b"id"),
            Err(e) => {
                error!("couldn't get id attribute value: {}", e);
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
                    error!("couldn't read id attribute value: {}", e);
                    Err(anyhow!("couldn't read id attribute value"))
                }
            }

        }
        None => Ok(None)
    }

}

fn get_element_text(reader: &mut Reader<&[u8]>, element_name: QName) -> Result<String, Error> {
    let txt = reader
        .read_text(element_name)?;
    Ok(txt.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fake::{Fake, Faker};

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
                },
                MetaProperty {
                    id: 2,
                    name: "CLI".to_string(),
                }
            ],
            clients: vec![
                MetaProperty {
                    id: 1,
                    name: "BirchStore".to_string(),
                },
                MetaProperty {
                    id: 2,
                    name: "KalinkaStore".to_string(),
                }
            ],
            accounts: vec![
                Account {
                    id: 1,
                    name: "Ivan Petrov".to_string(),
                    client_id: 1,
                    category_id: 2,
                    login: "i.petrov".to_string(),
                },
                Account {
                    id: 2,
                    name: "Abramova Nina".to_string(),
                    client_id: 2,
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

    #[test]
    fn return_error_for_missing_file() {
        let filename = Faker.fake::<String>();
        let path = Path::new(&filename);
        assert!(get_xml_config_from_file(&path).is_err());
    }

    #[test]
    fn return_error_for_invalid_xml_file() {
        let path = Path::new("test-data").join("invalid.xml");
        assert!(get_xml_config_from_file(path.as_path()).is_err());
    }
}

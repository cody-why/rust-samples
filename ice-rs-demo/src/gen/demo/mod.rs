use async_trait::async_trait;
use ice_rs::encoding::*;
use ice_rs::errors::*;
use ice_rs::iceobject::*;
use ice_rs::protocol::*;
use ice_rs::proxy::Proxy;
use num_enum::TryFromPrimitive;
use std::collections::HashMap;
use std::convert::TryFrom;
#[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]
#[repr(i32)]
pub enum NumberType {
    HOME = 0i32,
    OFFICE = 1i32,
    CELL = 2i32,
}
impl OptionalType for NumberType {
    fn optional_type() -> u8 {
        4
    }
}
impl ToBytes for NumberType {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        bytes.extend(IceSize { size: *self as i32 }.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for NumberType {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    where
        Self: Sized,
    {
        let mut read = 0;
        let enum_value = IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;
        *read_bytes = *read_bytes + read;
        match NumberType::try_from(enum_value) {
            Ok(enum_type) => Ok(enum_type),
            _ => Err(Box::new(ProtocolError::new(&format!(
                "Cannot convert int {} to enum",
                enum_value
            )))),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub name: String,
    pub num_type: Option<NumberType>,
    pub number: Option<String>,
    pub dial_group: Option<i32>,
}
impl ToBytes for Contact {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        let slice_flags = SliceFlags {
            type_id: SliceFlagsTypeEncoding::StringTypeId,
            optional_members: true,
            indirection_table: false,
            slice_size: false,
            last_slice: true,
        };
        bytes.extend(1u8.to_bytes()?);
        bytes.extend(slice_flags.to_bytes()?);
        bytes.extend("::Demo::Contact".to_bytes()?);
        bytes.extend(self.name.to_bytes()?);
        bytes.extend(OptionalWrapper::new(1u8, self.num_type.clone()).to_bytes()?);
        bytes.extend(OptionalWrapper::new(2u8, self.number.clone()).to_bytes()?);
        bytes.extend(OptionalWrapper::new(3u8, self.dial_group.clone()).to_bytes()?);
        bytes.extend(255u8.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for Contact {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    where
        Self: Sized,
    {
        let mut read = 0;
        let marker = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        if marker != 1 && marker != 255 {
            read = 0;
        }
        let flags = SliceFlags::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        match flags.type_id {
            SliceFlagsTypeEncoding::StringTypeId => {
                let _slice_name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
            },
            SliceFlagsTypeEncoding::CompactTypeId => {
                todo!()
            },
            SliceFlagsTypeEncoding::IndexTypeId => {
                todo!()
            },
            SliceFlagsTypeEncoding::NoTypeId => {},
        }
        let name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let mut num_type = None;
        let mut number = None;
        let mut dial_group = None;
        while read < bytes.len() as i32 {
            let flag_byte = bytes[read as usize..bytes.len()].first().unwrap();
            if *flag_byte == 0xFF {
                break;
            }
            let flag = OptionalFlag::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
            match flag.tag {
                1u8 => {
                    num_type = Some(NumberType::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);
                },
                2u8 => {
                    number = Some(String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);
                },
                3u8 => {
                    dial_group = Some(i32::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);
                },
                _ => {
                    if flags.last_slice {
                        return Err(Box::new(ProtocolError::new("Last slice not expected")));
                    } else {
                        read = read - 1;
                        break;
                    }
                },
            }
        }
        let obj = Self {
            name: name,
            num_type: num_type,
            number: number,
            dial_group: dial_group,
        };
        *read_bytes = *read_bytes + read;
        Ok(obj)
    }
}
#[async_trait]
pub trait ContactDB: IceObject {
    async fn add_contact(
        &mut self,
        name: &String,
        num_type: Option<NumberType>,
        number: Option<String>,
        dial_group: Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update_contact(
        &mut self,
        name: &String,
        num_type: Option<NumberType>,
        number: Option<String>,
        dial_group: Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn query(
        &mut self,
        name: &String,
        context: Option<HashMap<String, String>>,
    ) -> Result<Contact, Box<dyn std::error::Error + Send + Sync>>;
    async fn query_number(
        &mut self,
        name: &String,
        context: Option<HashMap<String, String>>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>;
    async fn query_dialgroup(
        &mut self,
        name: &String,
        dial_group: &mut Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn shutdown(
        &mut self,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
#[async_trait]
pub trait ContactDBI {
    async fn add_contact(
        &mut self,
        name: &String,
        num_type: Option<NumberType>,
        number: Option<String>,
        dial_group: Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> ();
    async fn update_contact(
        &mut self,
        name: &String,
        num_type: Option<NumberType>,
        number: Option<String>,
        dial_group: Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> ();
    async fn query(&mut self, name: &String, context: Option<HashMap<String, String>>) -> Contact;
    async fn query_number(&mut self, name: &String, context: Option<HashMap<String, String>>) -> Option<String>;
    async fn query_dialgroup(
        &mut self,
        name: &String,
        dial_group: &mut Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> ();
    async fn shutdown(&mut self, context: Option<HashMap<String, String>>) -> ();
}
pub struct ContactDBServer {
    server_impl: Box<dyn ContactDBI + Send + Sync>,
}
impl ContactDBServer {
    #[allow(dead_code)]
    pub fn new(server_impl: Box<dyn ContactDBI + Send + Sync>) -> ContactDBServer {
        ContactDBServer { server_impl }
    }
    async fn ice_is_a(&self, param: &str) -> bool {
        param == "::Demo::ContactDB"
    }
}
#[async_trait]
impl IceObjectServer for ContactDBServer {
    async fn handle_request(
        &mut self,
        request: &RequestData,
    ) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>> {
        match request.operation.as_ref() {
            "ice_isA" => {
                let mut read = 0;
                let param = String::from_bytes(&request.params.data, &mut read)?;
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(self.ice_is_a(&param).await.to_bytes()?),
                })
            },
            "addContact" => {
                let mut read_bytes = 0;
                let name = String::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut read_bytes,
                )?;
                let mut num_type = None;
                let mut number = None;
                let mut dial_group = None;
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 1u8 {
                            num_type = Option::<NumberType>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 2u8 {
                            number = Option::<String>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 3u8 {
                            dial_group = Option::<i32>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let result = self
                    .server_impl
                    .add_contact(&name, num_type, number, dial_group, None)
                    .await;
                let wrapped_result = result;
                let result = wrapped_result.to_bytes()?;
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result),
                })
            },
            "updateContact" => {
                let mut read_bytes = 0;
                let name = String::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut read_bytes,
                )?;
                let mut num_type = None;
                let mut number = None;
                let mut dial_group = None;
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 1u8 {
                            num_type = Option::<NumberType>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 2u8 {
                            number = Option::<String>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 3u8 {
                            dial_group = Option::<i32>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let result = self
                    .server_impl
                    .update_contact(&name, num_type, number, dial_group, None)
                    .await;
                let wrapped_result = result;
                let result = wrapped_result.to_bytes()?;
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result),
                })
            },
            "query" => {
                let mut read_bytes = 0;
                let name = String::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut read_bytes,
                )?;
                let result = self.server_impl.query(&name, None).await;
                let wrapped_result = result;
                let result = wrapped_result.to_bytes()?;
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result),
                })
            },
            "queryNumber" => {
                let mut read_bytes = 0;
                let name = String::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut read_bytes,
                )?;
                let result = self.server_impl.query_number(&name, None).await;
                let wrapped_result = OptionalWrapper::new(1u8, result);
                let result = wrapped_result.to_bytes()?;
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result),
                })
            },
            "queryDialgroup" => {
                let mut read_bytes = 0;
                let name = String::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut read_bytes,
                )?;
                let mut dial_group = None;
                let mut flag_bytes = 0;
                match OptionalFlag::from_bytes(
                    &request.params.data[read_bytes as usize..request.params.data.len()],
                    &mut flag_bytes,
                ) {
                    Ok(flag) => {
                        if flag.tag == 1u8 {
                            dial_group = Option::<i32>::from_bytes(
                                &request.params.data[read_bytes as usize..request.params.data.len()],
                                &mut read_bytes,
                            )?;
                        }
                    },
                    _ => {},
                }
                let result = self.server_impl.query_dialgroup(&name, &mut dial_group, None).await;
                let wrapped_result = result;
                let mut result = wrapped_result.to_bytes()?;
                result.extend(OptionalWrapper::new(1u8, dial_group).to_bytes()?);
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result),
                })
            },
            "shutdown" => {
                let result = self.server_impl.shutdown(None).await;
                let wrapped_result = result;
                let result = wrapped_result.to_bytes()?;
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result),
                })
            },
            _ => Err(Box::new(ProtocolError::new("Operation not found"))),
        }
    }
}
pub struct ContactDBPrx {
    pub proxy: Proxy,
}
#[async_trait]
impl IceObject for ContactDBPrx {
    async fn ice_ping(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        self.proxy
            .dispatch::<ProtocolError>(&String::from("ice_ping"), 1, &Encapsulation::empty(), None)
            .await?;
        Ok(())
    }
    async fn ice_is_a(&mut self) -> Result<bool, Box<dyn std::error::Error + Sync + Send>> {
        let reply = self
            .proxy
            .dispatch::<ProtocolError>(
                &String::from("ice_isA"),
                1,
                &Encapsulation::from(String::from("::Demo::ContactDB").to_bytes()?),
                None,
            )
            .await?;
        let mut read_bytes: i32 = 0;
        bool::from_bytes(&reply.body.data, &mut read_bytes)
    }
    async fn ice_id(&mut self) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        let reply = self
            .proxy
            .dispatch::<ProtocolError>(&String::from("ice_id"), 1, &Encapsulation::empty(), None)
            .await?;
        let mut read_bytes: i32 = 0;
        String::from_bytes(&reply.body.data, &mut read_bytes)
    }
    async fn ice_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
        let reply = self
            .proxy
            .dispatch::<ProtocolError>(&String::from("ice_ids"), 1, &Encapsulation::empty(), None)
            .await?;
        let mut read_bytes: i32 = 0;
        Vec::from_bytes(&reply.body.data, &mut read_bytes)
    }
}
#[async_trait]
impl ContactDB for ContactDBPrx {
    async fn add_contact(
        &mut self,
        name: &String,
        num_type: Option<NumberType>,
        number: Option<String>,
        dial_group: Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        bytes.extend(name.to_bytes()?);
        if let Some(value) = num_type {
            bytes.extend(OptionalFlag::new(1u8, NumberType::optional_type()).to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        if let Some(value) = number {
            bytes.extend(OptionalFlag::new(2u8, String::optional_type()).to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        if let Some(value) = dial_group {
            bytes.extend(OptionalFlag::new(3u8, i32::optional_type()).to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        self.proxy
            .dispatch::<ProtocolError>(&String::from("addContact"), 0u8, &Encapsulation::from(bytes), context)
            .await?;
        Ok(())
    }
    async fn update_contact(
        &mut self,
        name: &String,
        num_type: Option<NumberType>,
        number: Option<String>,
        dial_group: Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        bytes.extend(name.to_bytes()?);
        if let Some(value) = num_type {
            bytes.extend(OptionalFlag::new(1u8, NumberType::optional_type()).to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        if let Some(value) = number {
            bytes.extend(OptionalFlag::new(2u8, String::optional_type()).to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        if let Some(value) = dial_group {
            bytes.extend(OptionalFlag::new(3u8, i32::optional_type()).to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        self.proxy
            .dispatch::<ProtocolError>(
                &String::from("updateContact"),
                0u8,
                &Encapsulation::from(bytes),
                context,
            )
            .await?;
        Ok(())
    }
    async fn query(
        &mut self,
        name: &String,
        context: Option<HashMap<String, String>>,
    ) -> Result<Contact, Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        bytes.extend(name.to_bytes()?);
        let reply = self
            .proxy
            .dispatch::<ProtocolError>(&String::from("query"), 0u8, &Encapsulation::from(bytes), context)
            .await?;
        let mut read_bytes: i32 = 0;
        Contact::from_bytes(
            &reply.body.data[read_bytes as usize..reply.body.data.len()],
            &mut read_bytes,
        )
    }
    async fn query_number(
        &mut self,
        name: &String,
        context: Option<HashMap<String, String>>,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        bytes.extend(name.to_bytes()?);
        let reply = self
            .proxy
            .dispatch::<ProtocolError>(&String::from("queryNumber"), 0u8, &Encapsulation::from(bytes), context)
            .await?;
        let mut read_bytes: i32 = 0;
        Option::<String>::from_bytes(
            &reply.body.data[read_bytes as usize..reply.body.data.len()],
            &mut read_bytes,
        )
    }
    async fn query_dialgroup(
        &mut self,
        name: &String,
        dial_group: &mut Option<i32>,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut bytes = Vec::new();
        bytes.extend(name.to_bytes()?);
        let reply = self
            .proxy
            .dispatch::<ProtocolError>(
                &String::from("queryDialgroup"),
                0u8,
                &Encapsulation::from(bytes),
                context,
            )
            .await?;
        let mut read_bytes: i32 = 0;
        *dial_group = Option::<i32>::from_bytes(
            &reply.body.data[read_bytes as usize..reply.body.data.len()],
            &mut read_bytes,
        )?;
        Ok(())
    }
    async fn shutdown(
        &mut self,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bytes = Vec::new();
        self.proxy
            .dispatch::<ProtocolError>(&String::from("shutdown"), 0u8, &Encapsulation::from(bytes), context)
            .await?;
        Ok(())
    }
}
impl ContactDBPrx {
    #[allow(dead_code)]
    pub async fn unchecked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self { proxy: proxy })
    }
    #[allow(dead_code)]
    pub async fn checked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut my_proxy = Self::unchecked_cast(proxy).await?;
        if !my_proxy.ice_is_a().await? {
            return Err(Box::new(ProtocolError::new("ice_is_a() failed")));
        }
        Ok(my_proxy)
    }
}

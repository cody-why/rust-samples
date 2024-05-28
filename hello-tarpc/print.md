cargo expand 展开宏代码

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod service {
    trait World: Sized {
        ///The response future returned by [`World::hello`].
        type HelloFut: std::future::Future<Output = String>;
        fn hello(
            self,
            context: tarpc::context::Context,
            name: String,
            msg: String,
        ) -> Self::HelloFut;
        /// Returns a serving function to use with
        /// [InFlightRequest::execute](tarpc::server::InFlightRequest::execute).
        fn serve(self) -> ServeWorld<Self> {
            ServeWorld { service: self }
        }
    }
    /// A serving function to use with [tarpc::server::InFlightRequest::execute].
    struct ServeWorld<S> {
        service: S,
    }
    #[automatically_derived]
    impl<S: ::core::clone::Clone> ::core::clone::Clone for ServeWorld<S> {
        #[inline]
        fn clone(&self) -> ServeWorld<S> {
            ServeWorld {
                service: ::core::clone::Clone::clone(&self.service),
            }
        }
    }
    impl<S> tarpc::server::Serve<WorldRequest> for ServeWorld<S>
    where
        S: World,
    {
        type Resp = WorldResponse;
        type Fut = WorldResponseFut<S>;
        fn method(&self, req: &WorldRequest) -> Option<&'static str> {
            Some(
                match req {
                    WorldRequest::Hello { .. } => "World.hello",
                },
            )
        }
        fn serve(self, ctx: tarpc::context::Context, req: WorldRequest) -> Self::Fut {
            match req {
                WorldRequest::Hello { name, msg } => {
                    WorldResponseFut::Hello(World::hello(self.service, ctx, name, msg))
                }
            }
        }
    }
    /// The request sent over the wire from the client to the server.
    #[allow(missing_docs)]
    #[serde(crate = "tarpc::serde")]
    enum WorldRequest {
        Hello { name: String, msg: String },
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl tarpc::serde::Serialize for WorldRequest {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> tarpc::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: tarpc::serde::Serializer,
            {
                match *self {
                    WorldRequest::Hello { ref name, ref msg } => {
                        let mut __serde_state = match _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "WorldRequest",
                            0u32,
                            "Hello",
                            0 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "name",
                            name,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "msg",
                            msg,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl<'de> tarpc::serde::Deserialize<'de> for WorldRequest {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> tarpc::serde::__private::Result<Self, __D::Error>
            where
                __D: tarpc::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Hello" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Hello" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WorldRequest>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WorldRequest;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum WorldRequest",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private::Ok(__Field::__field0),
                                            1u64 => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "name" => _serde::__private::Ok(__Field::__field0),
                                            "msg" => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"name" => _serde::__private::Ok(__Field::__field0),
                                            b"msg" => _serde::__private::Ok(__Field::__field1),
                                            _ => _serde::__private::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<WorldRequest>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = WorldRequest;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "struct variant WorldRequest::Hello",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                                            String,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant WorldRequest::Hello with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                                            String,
                                        >(&mut __seq) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        } {
                                            _serde::__private::Some(__value) => __value,
                                            _serde::__private::None => {
                                                return _serde::__private::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant WorldRequest::Hello with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private::Ok(WorldRequest::Hello {
                                            name: __field0,
                                            msg: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                        while let _serde::__private::Some(__key)
                                            = match _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private::Option::is_some(&__field0) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                        );
                                                    }
                                                    __field0 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            String,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private::Option::is_some(&__field1) {
                                                        return _serde::__private::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("msg"),
                                                        );
                                                    }
                                                    __field1 = _serde::__private::Some(
                                                        match _serde::de::MapAccess::next_value::<
                                                            String,
                                                        >(&mut __map) {
                                                            _serde::__private::Ok(__val) => __val,
                                                            _serde::__private::Err(__err) => {
                                                                return _serde::__private::Err(__err);
                                                            }
                                                        },
                                                    );
                                                }
                                                _ => {
                                                    let _ = match _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private::Some(__field0) => __field0,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("name") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private::Some(__field1) => __field1,
                                            _serde::__private::None => {
                                                match _serde::__private::de::missing_field("msg") {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            }
                                        };
                                        _serde::__private::Ok(WorldRequest::Hello {
                                            name: __field0,
                                            msg: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["name", "msg"];
                                _serde::de::VariantAccess::struct_variant(
                                    __variant,
                                    FIELDS,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<WorldRequest>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Hello"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "WorldRequest",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WorldRequest>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::fmt::Debug for WorldRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                WorldRequest::Hello { name: __self_0, msg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Hello",
                        "name",
                        &__self_0,
                        "msg",
                        &__self_1,
                    )
                }
            }
        }
    }
    /// The response sent over the wire from the server to the client.
    #[allow(missing_docs)]
    #[serde(crate = "tarpc::serde")]
    enum WorldResponse {
        Hello(String),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl tarpc::serde::Serialize for WorldResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> tarpc::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: tarpc::serde::Serializer,
            {
                match *self {
                    WorldResponse::Hello(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "WorldResponse",
                            0u32,
                            "Hello",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use tarpc::serde as _serde;
        #[automatically_derived]
        impl<'de> tarpc::serde::Deserialize<'de> for WorldResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> tarpc::serde::__private::Result<Self, __D::Error>
            where
                __D: tarpc::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Hello" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Hello" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WorldResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WorldResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum WorldResponse",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        String,
                                    >(__variant),
                                    WorldResponse::Hello,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Hello"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "WorldResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WorldResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(missing_docs)]
    impl ::core::fmt::Debug for WorldResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                WorldResponse::Hello(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Hello",
                        &__self_0,
                    )
                }
            }
        }
    }
    /// A future resolving to a server response.
    #[allow(missing_docs)]
    enum WorldResponseFut<S: World> {
        Hello(<S as World>::HelloFut),
    }
    impl<S: World> std::fmt::Debug for WorldResponseFut<S> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("WorldResponseFut").finish()
        }
    }
    impl<S: World> std::future::Future for WorldResponseFut<S> {
        type Output = WorldResponse;
        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<WorldResponse> {
            unsafe {
                match std::pin::Pin::get_unchecked_mut(self) {
                    WorldResponseFut::Hello(resp) => {
                        std::pin::Pin::new_unchecked(resp)
                            .poll(cx)
                            .map(WorldResponse::Hello)
                    }
                }
            }
        }
    }
    #[allow(unused)]
    /// The client stub that makes RPC calls to the server. All request methods return
    /// [Futures](std::future::Future).
    struct WorldClient(tarpc::client::Channel<WorldRequest, WorldResponse>);
    #[automatically_derived]
    #[allow(unused)]
    impl ::core::clone::Clone for WorldClient {
        #[inline]
        fn clone(&self) -> WorldClient {
            WorldClient(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    #[allow(unused)]
    impl ::core::fmt::Debug for WorldClient {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WorldClient", &&self.0)
        }
    }
    impl WorldClient {
        /// Returns a new client stub that sends requests over the given transport.
        fn new<T>(
            config: tarpc::client::Config,
            transport: T,
        ) -> tarpc::client::NewClient<
            Self,
            tarpc::client::RequestDispatch<WorldRequest, WorldResponse, T>,
        >
        where
            T: tarpc::Transport<
                tarpc::ClientMessage<WorldRequest>,
                tarpc::Response<WorldResponse>,
            >,
        {
            let new_client = tarpc::client::new(config, transport);
            tarpc::client::NewClient {
                client: WorldClient(new_client.client),
                dispatch: new_client.dispatch,
            }
        }
    }
    impl WorldClient {
        #[allow(unused)]
        fn hello(
            &self,
            ctx: tarpc::context::Context,
            name: String,
            msg: String,
        ) -> impl std::future::Future<
            Output = Result<String, tarpc::client::RpcError>,
        > + '_ {
            let request = WorldRequest::Hello { name, msg };
            let resp = self.0.call(ctx, "World.hello", request);
            async move {
                match resp.await? {
                    WorldResponse::Hello(msg) => std::result::Result::Ok(msg),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
    }
    struct HelloServer(String);
    #[automatically_derived]
    impl ::core::clone::Clone for HelloServer {
        #[inline]
        fn clone(&self) -> HelloServer {
            HelloServer(::core::clone::Clone::clone(&self.0))
        }
    }
    impl World for HelloServer {
        fn hello(
            self,
            _: tarpc::context::Context,
            name: String,
            msg: String,
        ) -> ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = String> + ::core::marker::Send>,
        > {
            Box::pin(async move {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                let _ = (name, msg);
                "Hello".to_string()
            })
        }
        type HelloFut = ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = String> + ::core::marker::Send>,
        >;
    }
}
fn main() {}
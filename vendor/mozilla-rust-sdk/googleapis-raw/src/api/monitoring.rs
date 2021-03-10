// This file is generated by rust-protobuf 2.22.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/api/monitoring.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_0;

#[derive(PartialEq,Clone,Default)]
pub struct Monitoring {
    // message fields
    pub producer_destinations: ::protobuf::RepeatedField<Monitoring_MonitoringDestination>,
    pub consumer_destinations: ::protobuf::RepeatedField<Monitoring_MonitoringDestination>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Monitoring {
    fn default() -> &'a Monitoring {
        <Monitoring as ::protobuf::Message>::default_instance()
    }
}

impl Monitoring {
    pub fn new() -> Monitoring {
        ::std::default::Default::default()
    }

    // repeated .google.api.Monitoring.MonitoringDestination producer_destinations = 1;


    pub fn get_producer_destinations(&self) -> &[Monitoring_MonitoringDestination] {
        &self.producer_destinations
    }
    pub fn clear_producer_destinations(&mut self) {
        self.producer_destinations.clear();
    }

    // Param is passed by value, moved
    pub fn set_producer_destinations(&mut self, v: ::protobuf::RepeatedField<Monitoring_MonitoringDestination>) {
        self.producer_destinations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_producer_destinations(&mut self) -> &mut ::protobuf::RepeatedField<Monitoring_MonitoringDestination> {
        &mut self.producer_destinations
    }

    // Take field
    pub fn take_producer_destinations(&mut self) -> ::protobuf::RepeatedField<Monitoring_MonitoringDestination> {
        ::std::mem::replace(&mut self.producer_destinations, ::protobuf::RepeatedField::new())
    }

    // repeated .google.api.Monitoring.MonitoringDestination consumer_destinations = 2;


    pub fn get_consumer_destinations(&self) -> &[Monitoring_MonitoringDestination] {
        &self.consumer_destinations
    }
    pub fn clear_consumer_destinations(&mut self) {
        self.consumer_destinations.clear();
    }

    // Param is passed by value, moved
    pub fn set_consumer_destinations(&mut self, v: ::protobuf::RepeatedField<Monitoring_MonitoringDestination>) {
        self.consumer_destinations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_consumer_destinations(&mut self) -> &mut ::protobuf::RepeatedField<Monitoring_MonitoringDestination> {
        &mut self.consumer_destinations
    }

    // Take field
    pub fn take_consumer_destinations(&mut self) -> ::protobuf::RepeatedField<Monitoring_MonitoringDestination> {
        ::std::mem::replace(&mut self.consumer_destinations, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Monitoring {
    fn is_initialized(&self) -> bool {
        for v in &self.producer_destinations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.consumer_destinations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.producer_destinations)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.consumer_destinations)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.producer_destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.consumer_destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.producer_destinations {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.consumer_destinations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Monitoring {
        Monitoring::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Monitoring_MonitoringDestination>>(
                "producer_destinations",
                |m: &Monitoring| { &m.producer_destinations },
                |m: &mut Monitoring| { &mut m.producer_destinations },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Monitoring_MonitoringDestination>>(
                "consumer_destinations",
                |m: &Monitoring| { &m.consumer_destinations },
                |m: &mut Monitoring| { &mut m.consumer_destinations },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Monitoring>(
                "Monitoring",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Monitoring {
        static instance: ::protobuf::rt::LazyV2<Monitoring> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Monitoring::new)
    }
}

impl ::protobuf::Clear for Monitoring {
    fn clear(&mut self) {
        self.producer_destinations.clear();
        self.consumer_destinations.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Monitoring {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Monitoring {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Monitoring_MonitoringDestination {
    // message fields
    pub monitored_resource: ::std::string::String,
    pub metrics: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Monitoring_MonitoringDestination {
    fn default() -> &'a Monitoring_MonitoringDestination {
        <Monitoring_MonitoringDestination as ::protobuf::Message>::default_instance()
    }
}

impl Monitoring_MonitoringDestination {
    pub fn new() -> Monitoring_MonitoringDestination {
        ::std::default::Default::default()
    }

    // string monitored_resource = 1;


    pub fn get_monitored_resource(&self) -> &str {
        &self.monitored_resource
    }
    pub fn clear_monitored_resource(&mut self) {
        self.monitored_resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_monitored_resource(&mut self, v: ::std::string::String) {
        self.monitored_resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_monitored_resource(&mut self) -> &mut ::std::string::String {
        &mut self.monitored_resource
    }

    // Take field
    pub fn take_monitored_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.monitored_resource, ::std::string::String::new())
    }

    // repeated string metrics = 2;


    pub fn get_metrics(&self) -> &[::std::string::String] {
        &self.metrics
    }
    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    // Param is passed by value, moved
    pub fn set_metrics(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.metrics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metrics(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.metrics
    }

    // Take field
    pub fn take_metrics(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.metrics, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Monitoring_MonitoringDestination {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.monitored_resource)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.metrics)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.monitored_resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.monitored_resource);
        }
        for value in &self.metrics {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.monitored_resource.is_empty() {
            os.write_string(1, &self.monitored_resource)?;
        }
        for v in &self.metrics {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Monitoring_MonitoringDestination {
        Monitoring_MonitoringDestination::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "monitored_resource",
                |m: &Monitoring_MonitoringDestination| { &m.monitored_resource },
                |m: &mut Monitoring_MonitoringDestination| { &mut m.monitored_resource },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "metrics",
                |m: &Monitoring_MonitoringDestination| { &m.metrics },
                |m: &mut Monitoring_MonitoringDestination| { &mut m.metrics },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Monitoring_MonitoringDestination>(
                "Monitoring.MonitoringDestination",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Monitoring_MonitoringDestination {
        static instance: ::protobuf::rt::LazyV2<Monitoring_MonitoringDestination> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Monitoring_MonitoringDestination::new)
    }
}

impl ::protobuf::Clear for Monitoring_MonitoringDestination {
    fn clear(&mut self) {
        self.monitored_resource.clear();
        self.metrics.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Monitoring_MonitoringDestination {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Monitoring_MonitoringDestination {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/api/monitoring.proto\x12\ngoogle.api\"\xb4\x02\n\nMonitorin\
    g\x12a\n\x15producer_destinations\x18\x01\x20\x03(\x0b2,.google.api.Moni\
    toring.MonitoringDestinationR\x14producerDestinations\x12a\n\x15consumer\
    _destinations\x18\x02\x20\x03(\x0b2,.google.api.Monitoring.MonitoringDes\
    tinationR\x14consumerDestinations\x1a`\n\x15MonitoringDestination\x12-\n\
    \x12monitored_resource\x18\x01\x20\x01(\tR\x11monitoredResource\x12\x18\
    \n\x07metrics\x18\x02\x20\x03(\tR\x07metricsBq\n\x0ecom.google.apiB\x0fM\
    onitoringProtoP\x01ZEgoogle.golang.org/genproto/googleapis/api/serviceco\
    nfig;serviceconfig\xa2\x02\x04GAPIJ\xe2\x20\n\x06\x12\x04\x0e\0h\x01\n\
    \xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202020\x20Go\
    ogle\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Ver\
    sion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20th\
    is\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\x08\n\x01\x08\
    \x12\x03\x12\0\\\n\t\n\x02\x08\x0b\x12\x03\x12\0\\\n\x08\n\x01\x08\x12\
    \x03\x13\0\"\n\t\n\x02\x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\x12\x03\x14\
    \00\n\t\n\x02\x08\x08\x12\x03\x14\00\n\x08\n\x01\x08\x12\x03\x15\0'\n\t\
    \n\x02\x08\x01\x12\x03\x15\0'\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\
    \x08$\x12\x03\x16\0\"\n\x9e\x0f\n\x02\x04\0\x12\x04L\0h\x01\x1a\x91\x0f\
    \x20Monitoring\x20configuration\x20of\x20the\x20service.\n\n\x20The\x20e\
    xample\x20below\x20shows\x20how\x20to\x20configure\x20monitored\x20resou\
    rces\x20and\x20metrics\n\x20for\x20monitoring.\x20In\x20the\x20example,\
    \x20a\x20monitored\x20resource\x20and\x20two\x20metrics\x20are\n\x20defi\
    ned.\x20The\x20`library.googleapis.com/book/returned_count`\x20metric\
    \x20is\x20sent\n\x20to\x20both\x20producer\x20and\x20consumer\x20project\
    s,\x20whereas\x20the\n\x20`library.googleapis.com/book/num_overdue`\x20m\
    etric\x20is\x20only\x20sent\x20to\x20the\n\x20consumer\x20project.\n\n\
    \x20\x20\x20\x20\x20monitored_resources:\n\x20\x20\x20\x20\x20-\x20type:\
    \x20library.googleapis.com/Branch\n\x20\x20\x20\x20\x20\x20\x20display_n\
    ame:\x20\"Library\x20Branch\"\n\x20\x20\x20\x20\x20\x20\x20description:\
    \x20\"A\x20branch\x20of\x20a\x20library.\"\n\x20\x20\x20\x20\x20\x20\x20\
    launch_stage:\x20GA\n\x20\x20\x20\x20\x20\x20\x20labels:\n\x20\x20\x20\
    \x20\x20\x20\x20-\x20key:\x20resource_container\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20description:\x20\"The\x20Cloud\x20container\x20(ie.\x20p\
    roject\x20id)\x20for\x20the\x20Branch.\"\n\x20\x20\x20\x20\x20\x20\x20-\
    \x20key:\x20location\n\x20\x20\x20\x20\x20\x20\x20\x20\x20description:\
    \x20\"The\x20location\x20of\x20the\x20library\x20branch.\"\n\x20\x20\x20\
    \x20\x20\x20\x20-\x20key:\x20branch_id\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20description:\x20\"The\x20id\x20of\x20the\x20branch.\"\n\x20\x20\x20\
    \x20\x20metrics:\n\x20\x20\x20\x20\x20-\x20name:\x20library.googleapis.c\
    om/book/returned_count\n\x20\x20\x20\x20\x20\x20\x20display_name:\x20\"B\
    ooks\x20Returned\"\n\x20\x20\x20\x20\x20\x20\x20description:\x20\"The\
    \x20count\x20of\x20books\x20that\x20have\x20been\x20returned.\"\n\x20\
    \x20\x20\x20\x20\x20\x20launch_stage:\x20GA\n\x20\x20\x20\x20\x20\x20\
    \x20metric_kind:\x20DELTA\n\x20\x20\x20\x20\x20\x20\x20value_type:\x20IN\
    T64\n\x20\x20\x20\x20\x20\x20\x20unit:\x20\"1\"\n\x20\x20\x20\x20\x20\
    \x20\x20labels:\n\x20\x20\x20\x20\x20\x20\x20-\x20key:\x20customer_id\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20description:\x20\"The\x20id\x20of\
    \x20the\x20customer.\"\n\x20\x20\x20\x20\x20-\x20name:\x20library.google\
    apis.com/book/num_overdue\n\x20\x20\x20\x20\x20\x20\x20display_name:\x20\
    \"Books\x20Overdue\"\n\x20\x20\x20\x20\x20\x20\x20description:\x20\"The\
    \x20current\x20number\x20of\x20overdue\x20books.\"\n\x20\x20\x20\x20\x20\
    \x20\x20launch_stage:\x20GA\n\x20\x20\x20\x20\x20\x20\x20metric_kind:\
    \x20GAUGE\n\x20\x20\x20\x20\x20\x20\x20value_type:\x20INT64\n\x20\x20\
    \x20\x20\x20\x20\x20unit:\x20\"1\"\n\x20\x20\x20\x20\x20\x20\x20labels:\
    \n\x20\x20\x20\x20\x20\x20\x20-\x20key:\x20customer_id\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20description:\x20\"The\x20id\x20of\x20the\x20customer\
    .\"\n\x20\x20\x20\x20\x20monitoring:\n\x20\x20\x20\x20\x20\x20\x20produc\
    er_destinations:\n\x20\x20\x20\x20\x20\x20\x20-\x20monitored_resource:\
    \x20library.googleapis.com/Branch\n\x20\x20\x20\x20\x20\x20\x20\x20\x20m\
    etrics:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20library.googleapis.com\
    /book/returned_count\n\x20\x20\x20\x20\x20\x20\x20consumer_destinations:\
    \n\x20\x20\x20\x20\x20\x20\x20-\x20monitored_resource:\x20library.google\
    apis.com/Branch\n\x20\x20\x20\x20\x20\x20\x20\x20\x20metrics:\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20-\x20library.googleapis.com/book/returned_co\
    unt\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20library.googleapis.com/boo\
    k/num_overdue\n\n\n\n\x03\x04\0\x01\x12\x03L\x08\x12\ns\n\x04\x04\0\x03\
    \0\x12\x04O\x02W\x03\x1ae\x20Configuration\x20of\x20a\x20specific\x20mon\
    itoring\x20destination\x20(the\x20producer\x20project\n\x20or\x20the\x20\
    consumer\x20project).\n\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03O\n\x1f\n\x9a\
    \x01\n\x06\x04\0\x03\0\x02\0\x12\x03R\x04\"\x1a\x8a\x01\x20The\x20monito\
    red\x20resource\x20type.\x20The\x20type\x20must\x20be\x20defined\x20in\n\
    \x20[Service.monitored_resources][google.api.Service.monitored_resources\
    ]\x20section.\n\n\x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04R\x04O!\n\x0e\n\
    \x07\x04\0\x03\0\x02\0\x05\x12\x03R\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\
    \x01\x12\x03R\x0b\x1d\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03R\x20!\n\
    \xa5\x01\n\x06\x04\0\x03\0\x02\x01\x12\x03V\x04\x20\x1a\x95\x01\x20Types\
    \x20of\x20the\x20metrics\x20to\x20report\x20to\x20this\x20monitoring\x20\
    destination.\n\x20Each\x20type\x20must\x20be\x20defined\x20in\x20[Servic\
    e.metrics][google.api.Service.metrics]\x20section.\n\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x04\x12\x03V\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x01\x05\
    \x12\x03V\r\x13\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03V\x14\x1b\n\
    \x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03V\x1e\x1f\n\x9e\x03\n\x04\x04\
    \0\x02\0\x12\x03_\x02;\x1a\x90\x03\x20Monitoring\x20configurations\x20fo\
    r\x20sending\x20metrics\x20to\x20the\x20producer\x20project.\n\x20There\
    \x20can\x20be\x20multiple\x20producer\x20destinations.\x20A\x20monitored\
    \x20resource\x20type\x20may\n\x20appear\x20in\x20multiple\x20monitoring\
    \x20destinations\x20if\x20different\x20aggregations\x20are\n\x20needed\
    \x20for\x20different\x20sets\x20of\x20metrics\x20associated\x20with\x20t\
    hat\x20monitored\n\x20resource\x20type.\x20A\x20monitored\x20resource\
    \x20and\x20metric\x20pair\x20may\x20only\x20be\x20used\x20once\n\x20in\
    \x20the\x20Monitoring\x20configuration.\n\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03_\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03_\x0b\x20\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03_!6\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03_9:\n\x9e\
    \x03\n\x04\x04\0\x02\x01\x12\x03g\x02;\x1a\x90\x03\x20Monitoring\x20conf\
    igurations\x20for\x20sending\x20metrics\x20to\x20the\x20consumer\x20proj\
    ect.\n\x20There\x20can\x20be\x20multiple\x20consumer\x20destinations.\
    \x20A\x20monitored\x20resource\x20type\x20may\n\x20appear\x20in\x20multi\
    ple\x20monitoring\x20destinations\x20if\x20different\x20aggregations\x20\
    are\n\x20needed\x20for\x20different\x20sets\x20of\x20metrics\x20associat\
    ed\x20with\x20that\x20monitored\n\x20resource\x20type.\x20A\x20monitored\
    \x20resource\x20and\x20metric\x20pair\x20may\x20only\x20be\x20used\x20on\
    ce\n\x20in\x20the\x20Monitoring\x20configuration.\n\n\x0c\n\x05\x04\0\
    \x02\x01\x04\x12\x03g\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03g\x0b\
    \x20\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03g!6\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03g9:b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}

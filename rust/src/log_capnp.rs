// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: log.capnp

#![allow(unused_imports)]
#![allow(dead_code)]

pub mod h_t_t_p {
  use capnp::any_pointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
  use capnp::list::ToU16;

  pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 2, pointers : 4 };

  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_protocol(&self) -> Option<::log_capnp::h_t_t_p::protocol::Reader> {
      FromPrimitive::from_u16(self.reader.get_data_field::<u16>(0))
    }
    #[inline]
    pub fn get_status(&self) -> u16 {
      self.reader.get_data_field::<u16>(1)
    }
    #[inline]
    pub fn get_host_status(&self) -> u16 {
      self.reader.get_data_field::<u16>(2)
    }
    #[inline]
    pub fn get_up_status(&self) -> u16 {
      self.reader.get_data_field::<u16>(3)
    }
    #[inline]
    pub fn get_method(&self) -> Option<::log_capnp::h_t_t_p::method::Reader> {
      FromPrimitive::from_u16(self.reader.get_data_field::<u16>(4))
    }
    #[inline]
    pub fn get_content_type(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(0).get_text(::std::ptr::null(), 0)
    }
    pub fn has_content_type(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_user_agent(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(1).get_text(::std::ptr::null(), 0)
    }
    pub fn has_user_agent(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_referer(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(2).get_text(::std::ptr::null(), 0)
    }
    pub fn has_referer(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_request_u_r_i(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(3).get_text(::std::ptr::null(), 0)
    }
    pub fn has_request_u_r_i(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_protocol(&self) -> Option<::log_capnp::h_t_t_p::protocol::Reader> {
      FromPrimitive::from_u16(self.builder.get_data_field::<u16>(0))
    }
    #[inline]
    pub fn set_protocol(&self, value : ::log_capnp::h_t_t_p::protocol::Reader) {
      self.builder.set_data_field::<u16>(0, value as u16)
    }
    #[inline]
    pub fn get_status(&self) -> u16 {
      self.builder.get_data_field::<u16>(1)
    }
    #[inline]
    pub fn set_status(&self, value : u16) {
      self.builder.set_data_field::<u16>(1, value);
    }
    #[inline]
    pub fn get_host_status(&self) -> u16 {
      self.builder.get_data_field::<u16>(2)
    }
    #[inline]
    pub fn set_host_status(&self, value : u16) {
      self.builder.set_data_field::<u16>(2, value);
    }
    #[inline]
    pub fn get_up_status(&self) -> u16 {
      self.builder.get_data_field::<u16>(3)
    }
    #[inline]
    pub fn set_up_status(&self, value : u16) {
      self.builder.set_data_field::<u16>(3, value);
    }
    #[inline]
    pub fn get_method(&self) -> Option<::log_capnp::h_t_t_p::method::Reader> {
      FromPrimitive::from_u16(self.builder.get_data_field::<u16>(4))
    }
    #[inline]
    pub fn set_method(&self, value : ::log_capnp::h_t_t_p::method::Reader) {
      self.builder.set_data_field::<u16>(4, value as u16)
    }
    #[inline]
    pub fn get_content_type(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(0).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_content_type(&self, value : text::Reader) {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_content_type(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_content_type(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_user_agent(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(1).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_user_agent(&self, value : text::Reader) {
      self.builder.get_pointer_field(1).set_text(value);
    }
    #[inline]
    pub fn init_user_agent(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(1).init_text(size)
    }
    pub fn has_user_agent(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_referer(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(2).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_referer(&self, value : text::Reader) {
      self.builder.get_pointer_field(2).set_text(value);
    }
    #[inline]
    pub fn init_referer(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(2).init_text(size)
    }
    pub fn has_referer(&self) -> bool {
      !self.builder.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_request_u_r_i(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(3).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_request_u_r_i(&self, value : text::Reader) {
      self.builder.get_pointer_field(3).set_text(value);
    }
    #[inline]
    pub fn init_request_u_r_i(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(3).init_text(size)
    }
    pub fn has_request_u_r_i(&self) -> bool {
      !self.builder.get_pointer_field(3).is_null()
    }
  }

  pub struct Pipeline { _typeless : any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }

  pub mod protocol {
    use capnp::list::{ToU16};

    #[repr(u16)]
    #[deriving(FromPrimitive)]
    #[deriving(PartialEq)]
    pub enum Reader {
      Unknown = 0,
      Http10 = 1,
      Http11 = 2,
      Max = 3,
    }
    impl ToU16 for Reader {
      #[inline]
      fn to_u16(self) -> u16 { self as u16 }
    }
  }

  pub mod method {
    use capnp::list::{ToU16};

    #[repr(u16)]
    #[deriving(FromPrimitive)]
    #[deriving(PartialEq)]
    pub enum Reader {
      Unknown = 0,
      Get = 1,
      Post = 2,
      Delete = 3,
      Put = 4,
      Head = 5,
      Purge = 6,
      Options = 7,
      Propfind = 8,
      Mkcol = 9,
      Patch = 10,
      Max = 11,
    }
    impl ToU16 for Reader {
      #[inline]
      fn to_u16(self) -> u16 { self as u16 }
    }
  }
}

pub mod cache_status {
  use capnp::list::{ToU16};

  #[repr(u16)]
  #[deriving(FromPrimitive)]
  #[deriving(PartialEq)]
  pub enum Reader {
    Unknown = 0,
    Miss = 1,
    Expired = 2,
    Hit = 3,
    Max = 4,
  }
  impl ToU16 for Reader {
    #[inline]
    fn to_u16(self) -> u16 { self as u16 }
  }
}

pub mod origin {
  use capnp::any_pointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
  use capnp::list::ToU16;

  pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 1, pointers : 2 };

  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_ip(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(0).get_text(::std::ptr::null(), 0)
    }
    pub fn has_ip(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_port(&self) -> u16 {
      self.reader.get_data_field::<u16>(0)
    }
    #[inline]
    pub fn get_hostname(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(1).get_text(::std::ptr::null(), 0)
    }
    pub fn has_hostname(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_protocol(&self) -> Option<::log_capnp::origin::protocol::Reader> {
      FromPrimitive::from_u16(self.reader.get_data_field::<u16>(1))
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_ip(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(0).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_ip(&self, value : text::Reader) {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_ip(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_ip(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_port(&self) -> u16 {
      self.builder.get_data_field::<u16>(0)
    }
    #[inline]
    pub fn set_port(&self, value : u16) {
      self.builder.set_data_field::<u16>(0, value);
    }
    #[inline]
    pub fn get_hostname(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(1).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_hostname(&self, value : text::Reader) {
      self.builder.get_pointer_field(1).set_text(value);
    }
    #[inline]
    pub fn init_hostname(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(1).init_text(size)
    }
    pub fn has_hostname(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_protocol(&self) -> Option<::log_capnp::origin::protocol::Reader> {
      FromPrimitive::from_u16(self.builder.get_data_field::<u16>(1))
    }
    #[inline]
    pub fn set_protocol(&self, value : ::log_capnp::origin::protocol::Reader) {
      self.builder.set_data_field::<u16>(1, value as u16)
    }
  }

  pub struct Pipeline { _typeless : any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }

  pub mod protocol {
    use capnp::list::{ToU16};

    #[repr(u16)]
    #[deriving(FromPrimitive)]
    #[deriving(PartialEq)]
    pub enum Reader {
      Unknown = 0,
      Http = 1,
      Https = 2,
      Max = 3,
    }
    impl ToU16 for Reader {
      #[inline]
      fn to_u16(self) -> u16 { self as u16 }
    }
  }
}

pub mod zone_plan {
  use capnp::list::{ToU16};

  #[repr(u16)]
  #[deriving(FromPrimitive)]
  #[deriving(PartialEq)]
  pub enum Reader {
    Unknown = 0,
    Free = 1,
    Pro = 2,
    Biz = 3,
    Ent = 4,
    Max = 5,
  }
  impl ToU16 for Reader {
    #[inline]
    fn to_u16(self) -> u16 { self as u16 }
  }
}

pub mod log {
  use capnp::any_pointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
  use capnp::list::ToU16;

  pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 4, pointers : 6 };

  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_timestamp(&self) -> i64 {
      self.reader.get_data_field::<i64>(0)
    }
    #[inline]
    pub fn get_zone_id(&self) -> u32 {
      self.reader.get_data_field::<u32>(2)
    }
    #[inline]
    pub fn get_zone_plan(&self) -> Option<::log_capnp::zone_plan::Reader> {
      FromPrimitive::from_u16(self.reader.get_data_field::<u16>(6))
    }
    #[inline]
    pub fn get_http(&self) -> ::log_capnp::h_t_t_p::Reader<'a> {
      FromStructReader::new(self.reader.get_pointer_field(0).get_struct( ::std::ptr::null()))
    }
    pub fn has_http(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_origin(&self) -> ::log_capnp::origin::Reader<'a> {
      FromStructReader::new(self.reader.get_pointer_field(1).get_struct( ::std::ptr::null()))
    }
    pub fn has_origin(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_country(&self) -> Option<::country_capnp::country::Reader> {
      FromPrimitive::from_u16(self.reader.get_data_field::<u16>(7))
    }
    #[inline]
    pub fn get_cache_status(&self) -> Option<::log_capnp::cache_status::Reader> {
      FromPrimitive::from_u16(self.reader.get_data_field::<u16>(8))
    }
    #[inline]
    pub fn get_server_ip(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(2).get_text(::std::ptr::null(), 0)
    }
    pub fn has_server_ip(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_server_name(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(3).get_text(::std::ptr::null(), 0)
    }
    pub fn has_server_name(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_remote_ip(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(4).get_text(::std::ptr::null(), 0)
    }
    pub fn has_remote_ip(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_bytes_dlv(&self) -> u64 {
      self.reader.get_data_field::<u64>(3)
    }
    #[inline]
    pub fn get_ray_id(&self) -> text::Reader<'a> {
      self.reader.get_pointer_field(5).get_text(::std::ptr::null(), 0)
    }
    pub fn has_ray_id(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_timestamp(&self) -> i64 {
      self.builder.get_data_field::<i64>(0)
    }
    #[inline]
    pub fn set_timestamp(&self, value : i64) {
      self.builder.set_data_field::<i64>(0, value);
    }
    #[inline]
    pub fn get_zone_id(&self) -> u32 {
      self.builder.get_data_field::<u32>(2)
    }
    #[inline]
    pub fn set_zone_id(&self, value : u32) {
      self.builder.set_data_field::<u32>(2, value);
    }
    #[inline]
    pub fn get_zone_plan(&self) -> Option<::log_capnp::zone_plan::Reader> {
      FromPrimitive::from_u16(self.builder.get_data_field::<u16>(6))
    }
    #[inline]
    pub fn set_zone_plan(&self, value : ::log_capnp::zone_plan::Reader) {
      self.builder.set_data_field::<u16>(6, value as u16)
    }
    #[inline]
    pub fn get_http(&self) -> ::log_capnp::h_t_t_p::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(::log_capnp::h_t_t_p::STRUCT_SIZE, ::std::ptr::null()))
    }
    #[inline]
    pub fn set_http(&self, value : ::log_capnp::h_t_t_p::Reader) {
      self.builder.get_pointer_field(0).set_struct(&value.struct_reader())
    }
    #[inline]
    pub fn init_http(&self, ) -> ::log_capnp::h_t_t_p::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(::log_capnp::h_t_t_p::STRUCT_SIZE))
    }
    pub fn has_http(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_origin(&self) -> ::log_capnp::origin::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(1).get_struct(::log_capnp::origin::STRUCT_SIZE, ::std::ptr::null()))
    }
    #[inline]
    pub fn set_origin(&self, value : ::log_capnp::origin::Reader) {
      self.builder.get_pointer_field(1).set_struct(&value.struct_reader())
    }
    #[inline]
    pub fn init_origin(&self, ) -> ::log_capnp::origin::Builder<'a> {
      FromStructBuilder::new(self.builder.get_pointer_field(1).init_struct(::log_capnp::origin::STRUCT_SIZE))
    }
    pub fn has_origin(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_country(&self) -> Option<::country_capnp::country::Reader> {
      FromPrimitive::from_u16(self.builder.get_data_field::<u16>(7))
    }
    #[inline]
    pub fn set_country(&self, value : ::country_capnp::country::Reader) {
      self.builder.set_data_field::<u16>(7, value as u16)
    }
    #[inline]
    pub fn get_cache_status(&self) -> Option<::log_capnp::cache_status::Reader> {
      FromPrimitive::from_u16(self.builder.get_data_field::<u16>(8))
    }
    #[inline]
    pub fn set_cache_status(&self, value : ::log_capnp::cache_status::Reader) {
      self.builder.set_data_field::<u16>(8, value as u16)
    }
    #[inline]
    pub fn get_server_ip(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(2).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_server_ip(&self, value : text::Reader) {
      self.builder.get_pointer_field(2).set_text(value);
    }
    #[inline]
    pub fn init_server_ip(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(2).init_text(size)
    }
    pub fn has_server_ip(&self) -> bool {
      !self.builder.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_server_name(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(3).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_server_name(&self, value : text::Reader) {
      self.builder.get_pointer_field(3).set_text(value);
    }
    #[inline]
    pub fn init_server_name(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(3).init_text(size)
    }
    pub fn has_server_name(&self) -> bool {
      !self.builder.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_remote_ip(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(4).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_remote_ip(&self, value : text::Reader) {
      self.builder.get_pointer_field(4).set_text(value);
    }
    #[inline]
    pub fn init_remote_ip(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(4).init_text(size)
    }
    pub fn has_remote_ip(&self) -> bool {
      !self.builder.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_bytes_dlv(&self) -> u64 {
      self.builder.get_data_field::<u64>(3)
    }
    #[inline]
    pub fn set_bytes_dlv(&self, value : u64) {
      self.builder.set_data_field::<u64>(3, value);
    }
    #[inline]
    pub fn get_ray_id(&self) -> text::Builder<'a> {
      self.builder.get_pointer_field(5).get_text(::std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_ray_id(&self, value : text::Reader) {
      self.builder.get_pointer_field(5).set_text(value);
    }
    #[inline]
    pub fn init_ray_id(&self, size : u32) -> text::Builder<'a> {
      self.builder.get_pointer_field(5).init_text(size)
    }
    pub fn has_ray_id(&self) -> bool {
      !self.builder.get_pointer_field(5).is_null()
    }
  }

  pub struct Pipeline { _typeless : any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
    pub fn get_http(&self) -> ::log_capnp::h_t_t_p::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
    pub fn get_origin(&self) -> ::log_capnp::origin::Pipeline {
      FromTypelessPipeline::new(self._typeless.get_pointer_field(1))
    }
  }
}
// compile-flags: -O -C no-prepopulate-passes

#![crate_type = "lib"]
#![feature(rustc_attrs)]

use std::mem::MaybeUninit;
use std::num::NonZeroU64;

pub struct S {
  _field: [i32; 8],
}

pub struct UnsafeInner {
  _field: std::cell::UnsafeCell<i16>,
}

pub enum MyBool {
  True,
  False,
}

// CHECK: noundef zeroext i1 @boolean(i1 noundef zeroext %x)
#[no_mangle]
pub fn boolean(x: bool) -> bool {
  x
}

// CHECK: i8 @maybeuninit_boolean(i8 %x)
#[no_mangle]
pub fn maybeuninit_boolean(x: MaybeUninit<bool>) -> MaybeUninit<bool> {
  x
}

// CHECK: noundef zeroext i1 @enum_bool(i1 noundef zeroext %x)
#[no_mangle]
pub fn enum_bool(x: MyBool) -> MyBool {
  x
}

// CHECK: i8 @maybeuninit_enum_bool(i8 %x)
#[no_mangle]
pub fn maybeuninit_enum_bool(x: MaybeUninit<MyBool>) -> MaybeUninit<MyBool> {
  x
}

// CHECK: noundef i32 @char(i32 noundef %x)
#[no_mangle]
pub fn char(x: char) -> char {
  x
}

// CHECK: i32 @maybeuninit_char(i32 %x)
#[no_mangle]
pub fn maybeuninit_char(x: MaybeUninit<char>) -> MaybeUninit<char> {
  x
}

// CHECK: i64 @int(i64 %x)
#[no_mangle]
pub fn int(x: u64) -> u64 {
  x
}

// CHECK: noundef i64 @nonzero_int(i64 noundef %x)
#[no_mangle]
pub fn nonzero_int(x: NonZeroU64) -> NonZeroU64 {
  x
}

// CHECK: i64 @option_nonzero_int(i64 %x)
#[no_mangle]
pub fn option_nonzero_int(x: Option<NonZeroU64>) -> Option<NonZeroU64> {
  x
}

// CHECK: @readonly_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn readonly_borrow(_: &i32) {
}

// CHECK: @static_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1)
// static borrow may be captured
#[no_mangle]
pub fn static_borrow(_: &'static i32) {
}

// CHECK: @named_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1)
// borrow with named lifetime may be captured
#[no_mangle]
pub fn named_borrow<'r>(_: &'r i32) {
}

// CHECK: @unsafe_borrow(i16* noundef align 2 dereferenceable(2) %_1)
// unsafe interior means this isn't actually readonly and there may be aliases ...
#[no_mangle]
pub fn unsafe_borrow(_: &UnsafeInner) {
}

// CHECK: @mutable_unsafe_borrow(i16* noalias noundef align 2 dereferenceable(2) %_1)
// ... unless this is a mutable borrow, those never alias
#[no_mangle]
pub fn mutable_unsafe_borrow(_: &mut UnsafeInner) {
}

// CHECK: @mutable_borrow(i32* noalias noundef align 4 dereferenceable(4) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn mutable_borrow(_: &mut i32) {
}

// CHECK: @indirect_struct(%S* noalias nocapture noundef dereferenceable(32) %_1)
#[no_mangle]
pub fn indirect_struct(_: S) {
}

// CHECK: @borrowed_struct(%S* noalias noundef readonly align 4 dereferenceable(32) %_1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn borrowed_struct(_: &S) {
}

// CHECK: @raw_struct(%S* %_1)
#[no_mangle]
pub fn raw_struct(_: *const S) {
}

// `Box` can get deallocated during execution of the function, so it should
// not get `dereferenceable`.
// CHECK: noalias noundef nonnull align 4 i32* @_box(i32* noalias noundef nonnull align 4 %x)
#[no_mangle]
pub fn _box(x: Box<i32>) -> Box<i32> {
  x
}

// CHECK: @struct_return(%S* noalias nocapture noundef sret(%S) dereferenceable(32){{( %0)?}})
#[no_mangle]
pub fn struct_return() -> S {
  S {
    _field: [0, 0, 0, 0, 0, 0, 0, 0]
  }
}

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i[0-9]+]] %_1)
#[no_mangle]
pub fn helper(_: usize) {
}

// CHECK: @slice([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, [[USIZE]] %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn slice(_: &[u8]) {
}

// CHECK: @mutable_slice([0 x i8]* noalias noundef nonnull align 1 %_1.0, [[USIZE]] %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn mutable_slice(_: &mut [u8]) {
}

// CHECK: @unsafe_slice([0 x i16]* noundef nonnull align 2 %_1.0, [[USIZE]] %_1.1)
// unsafe interior means this isn't actually readonly and there may be aliases ...
#[no_mangle]
pub fn unsafe_slice(_: &[UnsafeInner]) {
}

// CHECK: @raw_slice([0 x i8]* %_1.0, [[USIZE]] %_1.1)
#[no_mangle]
pub fn raw_slice(_: *const [u8]) {
}

// CHECK: @str([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, [[USIZE]] %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn str(_: &[u8]) {
}

// CHECK: @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x [[USIZE]]]* noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn trait_borrow(_: &Drop) {
}

// CHECK: @trait_raw({}* %_1.0, [3 x [[USIZE]]]* noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
#[no_mangle]
pub fn trait_raw(_: *const Drop) {
}

// CHECK: @trait_box({}* noalias noundef nonnull align 1{{( %0)?}}, [3 x [[USIZE]]]* noalias noundef readonly align {{.*}} dereferenceable({{.*}}){{( %1)?}})
#[no_mangle]
pub fn trait_box(_: Box<Drop>) {
}

// CHECK: { i8*, i8* } @trait_option(i8* noalias noundef align 1 %x.0, i8* %x.1)
#[no_mangle]
pub fn trait_option(x: Option<Box<Drop>>) -> Option<Box<Drop>> {
  x
}

// CHECK: { [0 x i16]*, [[USIZE]] } @return_slice([0 x i16]* noalias noundef nonnull readonly align 2 %x.0, [[USIZE]] %x.1)
#[no_mangle]
pub fn return_slice(x: &[u16]) -> &[u16] {
  x
}

// CHECK: { i16, i16 } @enum_id_1(i16 noundef %x.0, i16 %x.1)
#[no_mangle]
pub fn enum_id_1(x: Option<Result<u16, u16>>) -> Option<Result<u16, u16>> {
  x
}

// CHECK: { i8, i8 } @enum_id_2(i1 noundef zeroext %x.0, i8 %x.1)
#[no_mangle]
pub fn enum_id_2(x: Option<u8>) -> Option<u8> {
  x
}

// CHECK: noalias i8* @allocator()
#[no_mangle]
#[rustc_allocator]
pub fn allocator() -> *const i8 {
  std::ptr::null()
}

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> core::ffi::c_long;
    fn ferror(__stream: *mut FILE) -> core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qoi_desc {
    pub width: core::ffi::c_uint,
    pub height: core::ffi::c_uint,
    pub channels: core::ffi::c_uchar,
    pub colorspace: core::ffi::c_uchar,
}
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub union qoi_rgba_t {
    pub rgba: C2RustUnnamed,
    pub v: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub r: core::ffi::c_uchar,
    pub g: core::ffi::c_uchar,
    pub b: core::ffi::c_uchar,
    pub a: core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const NULL_0: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const QOI_OP_INDEX: core::ffi::c_int = 0 as core::ffi::c_int;
pub const QOI_OP_DIFF: core::ffi::c_int = 0x40 as core::ffi::c_int;
pub const QOI_OP_LUMA: core::ffi::c_int = 0x80 as core::ffi::c_int;
pub const QOI_OP_RUN: core::ffi::c_int = 0xc0 as core::ffi::c_int;
pub const QOI_OP_RGB: core::ffi::c_int = 0xfe as core::ffi::c_int;
pub const QOI_OP_RGBA: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const QOI_MASK_2: core::ffi::c_int = 0xc0 as core::ffi::c_int;
pub const QOI_MAGIC: core::ffi::c_uint = ('q' as i32 as core::ffi::c_uint)
    << 24 as core::ffi::c_int
    | ('o' as i32 as core::ffi::c_uint) << 16 as core::ffi::c_int
    | ('i' as i32 as core::ffi::c_uint) << 8 as core::ffi::c_int
    | 'f' as i32 as core::ffi::c_uint;
pub const QOI_HEADER_SIZE: core::ffi::c_int = 14 as core::ffi::c_int;
pub const QOI_PIXELS_MAX: core::ffi::c_uint = 400000000 as core::ffi::c_int
    as core::ffi::c_uint;
static mut qoi_padding: [core::ffi::c_uchar; 8] = [
    0 as core::ffi::c_int as core::ffi::c_uchar,
    0 as core::ffi::c_int as core::ffi::c_uchar,
    0 as core::ffi::c_int as core::ffi::c_uchar,
    0 as core::ffi::c_int as core::ffi::c_uchar,
    0 as core::ffi::c_int as core::ffi::c_uchar,
    0 as core::ffi::c_int as core::ffi::c_uchar,
    0 as core::ffi::c_int as core::ffi::c_uchar,
    1 as core::ffi::c_int as core::ffi::c_uchar,
];
unsafe extern "C" fn qoi_write_32(
    mut bytes: *mut core::ffi::c_uchar,
    mut p: *mut core::ffi::c_int,
    mut v: core::ffi::c_uint,
) {
    let fresh0 = *p;
    *p = *p + 1;
    *bytes.offset(fresh0 as isize) = ((0xff000000 as core::ffi::c_uint & v)
        >> 24 as core::ffi::c_int) as core::ffi::c_uchar;
    let fresh1 = *p;
    *p = *p + 1;
    *bytes.offset(fresh1 as isize) = ((0xff0000 as core::ffi::c_uint & v)
        >> 16 as core::ffi::c_int) as core::ffi::c_uchar;
    let fresh2 = *p;
    *p = *p + 1;
    *bytes.offset(fresh2 as isize) = ((0xff00 as core::ffi::c_uint & v)
        >> 8 as core::ffi::c_int) as core::ffi::c_uchar;
    let fresh3 = *p;
    *p = *p + 1;
    *bytes.offset(fresh3 as isize) = (0xff as core::ffi::c_uint & v)
        as core::ffi::c_uchar;
}
unsafe extern "C" fn qoi_read_32(
    mut bytes: *const core::ffi::c_uchar,
    mut p: *mut core::ffi::c_int,
) -> core::ffi::c_uint {
    let fresh4 = *p;
    *p = *p + 1;
    let mut a: core::ffi::c_uint = *bytes.offset(fresh4 as isize) as core::ffi::c_uint;
    let fresh5 = *p;
    *p = *p + 1;
    let mut b: core::ffi::c_uint = *bytes.offset(fresh5 as isize) as core::ffi::c_uint;
    let fresh6 = *p;
    *p = *p + 1;
    let mut c: core::ffi::c_uint = *bytes.offset(fresh6 as isize) as core::ffi::c_uint;
    let fresh7 = *p;
    *p = *p + 1;
    let mut d: core::ffi::c_uint = *bytes.offset(fresh7 as isize) as core::ffi::c_uint;
    return a << 24 as core::ffi::c_int | b << 16 as core::ffi::c_int
        | c << 8 as core::ffi::c_int | d;
}
#[no_mangle]
pub unsafe extern "C" fn qoi_encode(
    mut data: *const core::ffi::c_void,
    mut desc: *const qoi_desc,
    mut out_len: *mut core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut i: core::ffi::c_int = 0;
    let mut max_size: core::ffi::c_int = 0;
    let mut p: core::ffi::c_int = 0;
    let mut run: core::ffi::c_int = 0;
    let mut px_len: core::ffi::c_int = 0;
    let mut px_end: core::ffi::c_int = 0;
    let mut px_pos: core::ffi::c_int = 0;
    let mut channels: core::ffi::c_int = 0;
    let mut bytes: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut pixels: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut index: [qoi_rgba_t; 64] = [qoi_rgba_t {
        rgba: C2RustUnnamed {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    }; 64];
    let mut px: qoi_rgba_t = qoi_rgba_t {
        rgba: C2RustUnnamed {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    };
    let mut px_prev: qoi_rgba_t = qoi_rgba_t {
        rgba: C2RustUnnamed {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    };
    if data.is_null() || out_len.is_null() || desc.is_null()
        || (*desc).width == 0 as core::ffi::c_uint
        || (*desc).height == 0 as core::ffi::c_uint
        || ((*desc).channels as core::ffi::c_int) < 3 as core::ffi::c_int
        || (*desc).channels as core::ffi::c_int > 4 as core::ffi::c_int
        || (*desc).colorspace as core::ffi::c_int > 1 as core::ffi::c_int
        || (*desc).height >= QOI_PIXELS_MAX.wrapping_div((*desc).width)
    {
        return NULL;
    }
    max_size = (((*desc).width)
        .wrapping_mul((*desc).height)
        .wrapping_mul(
            ((*desc).channels as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_uint,
        )
        .wrapping_add(QOI_HEADER_SIZE as core::ffi::c_uint) as usize)
        .wrapping_add(::core::mem::size_of::<[core::ffi::c_uchar; 8]>() as usize)
        as core::ffi::c_int;
    p = 0 as core::ffi::c_int;
    bytes = malloc(max_size as size_t) as *mut core::ffi::c_uchar;
    if bytes.is_null() {
        return NULL;
    }
    qoi_write_32(bytes, &mut p, QOI_MAGIC);
    qoi_write_32(bytes, &mut p, (*desc).width);
    qoi_write_32(bytes, &mut p, (*desc).height);
    let fresh8 = p;
    p = p + 1;
    *bytes.offset(fresh8 as isize) = (*desc).channels;
    let fresh9 = p;
    p = p + 1;
    *bytes.offset(fresh9 as isize) = (*desc).colorspace;
    pixels = data as *const core::ffi::c_uchar;
    memset(
        index.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[qoi_rgba_t; 64]>() as size_t,
    );
    run = 0 as core::ffi::c_int;
    px_prev.rgba.r = 0 as core::ffi::c_uchar;
    px_prev.rgba.g = 0 as core::ffi::c_uchar;
    px_prev.rgba.b = 0 as core::ffi::c_uchar;
    px_prev.rgba.a = 255 as core::ffi::c_uchar;
    px = px_prev;
    px_len = ((*desc).width)
        .wrapping_mul((*desc).height)
        .wrapping_mul((*desc).channels as core::ffi::c_uint) as core::ffi::c_int;
    px_end = px_len - (*desc).channels as core::ffi::c_int;
    channels = (*desc).channels as core::ffi::c_int;
    px_pos = 0 as core::ffi::c_int;
    while px_pos < px_len {
        px.rgba.r = *pixels.offset((px_pos + 0 as core::ffi::c_int) as isize);
        px.rgba.g = *pixels.offset((px_pos + 1 as core::ffi::c_int) as isize);
        px.rgba.b = *pixels.offset((px_pos + 2 as core::ffi::c_int) as isize);
        if channels == 4 as core::ffi::c_int {
            px.rgba.a = *pixels.offset((px_pos + 3 as core::ffi::c_int) as isize);
        }
        if px.v == px_prev.v {
            run += 1;
            if run == 62 as core::ffi::c_int || px_pos == px_end {
                let fresh10 = p;
                p = p + 1;
                *bytes.offset(fresh10 as isize) = (QOI_OP_RUN
                    | run - 1 as core::ffi::c_int) as core::ffi::c_uchar;
                run = 0 as core::ffi::c_int;
            }
        } else {
            let mut index_pos: core::ffi::c_int = 0;
            if run > 0 as core::ffi::c_int {
                let fresh11 = p;
                p = p + 1;
                *bytes.offset(fresh11 as isize) = (QOI_OP_RUN
                    | run - 1 as core::ffi::c_int) as core::ffi::c_uchar;
                run = 0 as core::ffi::c_int;
            }
            index_pos = px.rgba.r as core::ffi::c_int * 3 as core::ffi::c_int
                + px.rgba.g as core::ffi::c_int * 5 as core::ffi::c_int
                + px.rgba.b as core::ffi::c_int * 7 as core::ffi::c_int
                + px.rgba.a as core::ffi::c_int * 11 as core::ffi::c_int
                & 64 as core::ffi::c_int - 1 as core::ffi::c_int;
            if index[index_pos as usize].v == px.v {
                let fresh12 = p;
                p = p + 1;
                *bytes.offset(fresh12 as isize) = (QOI_OP_INDEX | index_pos)
                    as core::ffi::c_uchar;
            } else {
                index[index_pos as usize] = px;
                if px.rgba.a as core::ffi::c_int == px_prev.rgba.a as core::ffi::c_int {
                    let mut vr: core::ffi::c_schar = (px.rgba.r as core::ffi::c_int
                        - px_prev.rgba.r as core::ffi::c_int) as core::ffi::c_schar;
                    let mut vg: core::ffi::c_schar = (px.rgba.g as core::ffi::c_int
                        - px_prev.rgba.g as core::ffi::c_int) as core::ffi::c_schar;
                    let mut vb: core::ffi::c_schar = (px.rgba.b as core::ffi::c_int
                        - px_prev.rgba.b as core::ffi::c_int) as core::ffi::c_schar;
                    let mut vg_r: core::ffi::c_schar = (vr as core::ffi::c_int
                        - vg as core::ffi::c_int) as core::ffi::c_schar;
                    let mut vg_b: core::ffi::c_schar = (vb as core::ffi::c_int
                        - vg as core::ffi::c_int) as core::ffi::c_schar;
                    if vr as core::ffi::c_int > -(3 as core::ffi::c_int)
                        && (vr as core::ffi::c_int) < 2 as core::ffi::c_int
                        && vg as core::ffi::c_int > -(3 as core::ffi::c_int)
                        && (vg as core::ffi::c_int) < 2 as core::ffi::c_int
                        && vb as core::ffi::c_int > -(3 as core::ffi::c_int)
                        && (vb as core::ffi::c_int) < 2 as core::ffi::c_int
                    {
                        let fresh13 = p;
                        p = p + 1;
                        *bytes.offset(fresh13 as isize) = (QOI_OP_DIFF
                            | (vr as core::ffi::c_int + 2 as core::ffi::c_int)
                                << 4 as core::ffi::c_int
                            | (vg as core::ffi::c_int + 2 as core::ffi::c_int)
                                << 2 as core::ffi::c_int
                            | vb as core::ffi::c_int + 2 as core::ffi::c_int)
                            as core::ffi::c_uchar;
                    } else if vg_r as core::ffi::c_int > -(9 as core::ffi::c_int)
                        && (vg_r as core::ffi::c_int) < 8 as core::ffi::c_int
                        && vg as core::ffi::c_int > -(33 as core::ffi::c_int)
                        && (vg as core::ffi::c_int) < 32 as core::ffi::c_int
                        && vg_b as core::ffi::c_int > -(9 as core::ffi::c_int)
                        && (vg_b as core::ffi::c_int) < 8 as core::ffi::c_int
                    {
                        let fresh14 = p;
                        p = p + 1;
                        *bytes.offset(fresh14 as isize) = (QOI_OP_LUMA
                            | vg as core::ffi::c_int + 32 as core::ffi::c_int)
                            as core::ffi::c_uchar;
                        let fresh15 = p;
                        p = p + 1;
                        *bytes.offset(fresh15 as isize) = ((vg_r as core::ffi::c_int
                            + 8 as core::ffi::c_int) << 4 as core::ffi::c_int
                            | vg_b as core::ffi::c_int + 8 as core::ffi::c_int)
                            as core::ffi::c_uchar;
                    } else {
                        let fresh16 = p;
                        p = p + 1;
                        *bytes.offset(fresh16 as isize) = QOI_OP_RGB
                            as core::ffi::c_uchar;
                        let fresh17 = p;
                        p = p + 1;
                        *bytes.offset(fresh17 as isize) = px.rgba.r;
                        let fresh18 = p;
                        p = p + 1;
                        *bytes.offset(fresh18 as isize) = px.rgba.g;
                        let fresh19 = p;
                        p = p + 1;
                        *bytes.offset(fresh19 as isize) = px.rgba.b;
                    }
                } else {
                    let fresh20 = p;
                    p = p + 1;
                    *bytes.offset(fresh20 as isize) = QOI_OP_RGBA as core::ffi::c_uchar;
                    let fresh21 = p;
                    p = p + 1;
                    *bytes.offset(fresh21 as isize) = px.rgba.r;
                    let fresh22 = p;
                    p = p + 1;
                    *bytes.offset(fresh22 as isize) = px.rgba.g;
                    let fresh23 = p;
                    p = p + 1;
                    *bytes.offset(fresh23 as isize) = px.rgba.b;
                    let fresh24 = p;
                    p = p + 1;
                    *bytes.offset(fresh24 as isize) = px.rgba.a;
                }
            }
        }
        px_prev = px;
        px_pos += channels;
    }
    i = 0 as core::ffi::c_int;
    while i < ::core::mem::size_of::<[core::ffi::c_uchar; 8]>() as core::ffi::c_int {
        let fresh25 = p;
        p = p + 1;
        *bytes.offset(fresh25 as isize) = qoi_padding[i as usize];
        i += 1;
    }
    *out_len = p;
    return bytes as *mut core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn qoi_decode(
    mut data: *const core::ffi::c_void,
    mut size: core::ffi::c_int,
    mut desc: *mut qoi_desc,
    mut channels: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut bytes: *const core::ffi::c_uchar = 0 as *const core::ffi::c_uchar;
    let mut header_magic: core::ffi::c_uint = 0;
    let mut pixels: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    let mut index: [qoi_rgba_t; 64] = [qoi_rgba_t {
        rgba: C2RustUnnamed {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    }; 64];
    let mut px: qoi_rgba_t = qoi_rgba_t {
        rgba: C2RustUnnamed {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    };
    let mut px_len: core::ffi::c_int = 0;
    let mut chunks_len: core::ffi::c_int = 0;
    let mut px_pos: core::ffi::c_int = 0;
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut run: core::ffi::c_int = 0 as core::ffi::c_int;
    if data.is_null() || desc.is_null()
        || channels != 0 as core::ffi::c_int && channels != 3 as core::ffi::c_int
            && channels != 4 as core::ffi::c_int
        || size
            < QOI_HEADER_SIZE
                + ::core::mem::size_of::<[core::ffi::c_uchar; 8]>() as core::ffi::c_int
    {
        return NULL;
    }
    bytes = data as *const core::ffi::c_uchar;
    header_magic = qoi_read_32(bytes, &mut p);
    (*desc).width = qoi_read_32(bytes, &mut p);
    (*desc).height = qoi_read_32(bytes, &mut p);
    let fresh26 = p;
    p = p + 1;
    (*desc).channels = *bytes.offset(fresh26 as isize);
    let fresh27 = p;
    p = p + 1;
    (*desc).colorspace = *bytes.offset(fresh27 as isize);
    if (*desc).width == 0 as core::ffi::c_uint
        || (*desc).height == 0 as core::ffi::c_uint
        || ((*desc).channels as core::ffi::c_int) < 3 as core::ffi::c_int
        || (*desc).channels as core::ffi::c_int > 4 as core::ffi::c_int
        || (*desc).colorspace as core::ffi::c_int > 1 as core::ffi::c_int
        || header_magic != QOI_MAGIC
        || (*desc).height >= QOI_PIXELS_MAX.wrapping_div((*desc).width)
    {
        return NULL;
    }
    if channels == 0 as core::ffi::c_int {
        channels = (*desc).channels as core::ffi::c_int;
    }
    px_len = ((*desc).width)
        .wrapping_mul((*desc).height)
        .wrapping_mul(channels as core::ffi::c_uint) as core::ffi::c_int;
    pixels = malloc(px_len as size_t) as *mut core::ffi::c_uchar;
    if pixels.is_null() {
        return NULL;
    }
    memset(
        index.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[qoi_rgba_t; 64]>() as size_t,
    );
    px.rgba.r = 0 as core::ffi::c_uchar;
    px.rgba.g = 0 as core::ffi::c_uchar;
    px.rgba.b = 0 as core::ffi::c_uchar;
    px.rgba.a = 255 as core::ffi::c_uchar;
    chunks_len = size
        - ::core::mem::size_of::<[core::ffi::c_uchar; 8]>() as core::ffi::c_int;
    px_pos = 0 as core::ffi::c_int;
    while px_pos < px_len {
        if run > 0 as core::ffi::c_int {
            run -= 1;
        } else if p < chunks_len {
            let fresh28 = p;
            p = p + 1;
            let mut b1: core::ffi::c_int = *bytes.offset(fresh28 as isize)
                as core::ffi::c_int;
            if b1 == QOI_OP_RGB {
                let fresh29 = p;
                p = p + 1;
                px.rgba.r = *bytes.offset(fresh29 as isize);
                let fresh30 = p;
                p = p + 1;
                px.rgba.g = *bytes.offset(fresh30 as isize);
                let fresh31 = p;
                p = p + 1;
                px.rgba.b = *bytes.offset(fresh31 as isize);
            } else if b1 == QOI_OP_RGBA {
                let fresh32 = p;
                p = p + 1;
                px.rgba.r = *bytes.offset(fresh32 as isize);
                let fresh33 = p;
                p = p + 1;
                px.rgba.g = *bytes.offset(fresh33 as isize);
                let fresh34 = p;
                p = p + 1;
                px.rgba.b = *bytes.offset(fresh34 as isize);
                let fresh35 = p;
                p = p + 1;
                px.rgba.a = *bytes.offset(fresh35 as isize);
            } else if b1 & QOI_MASK_2 == QOI_OP_INDEX {
                px = index[b1 as usize];
            } else if b1 & QOI_MASK_2 == QOI_OP_DIFF {
                px.rgba.r = (px.rgba.r as core::ffi::c_int
                    + ((b1 >> 4 as core::ffi::c_int & 0x3 as core::ffi::c_int)
                        - 2 as core::ffi::c_int)) as core::ffi::c_uchar;
                px.rgba.g = (px.rgba.g as core::ffi::c_int
                    + ((b1 >> 2 as core::ffi::c_int & 0x3 as core::ffi::c_int)
                        - 2 as core::ffi::c_int)) as core::ffi::c_uchar;
                px.rgba.b = (px.rgba.b as core::ffi::c_int
                    + ((b1 & 0x3 as core::ffi::c_int) - 2 as core::ffi::c_int))
                    as core::ffi::c_uchar;
            } else if b1 & QOI_MASK_2 == QOI_OP_LUMA {
                let fresh36 = p;
                p = p + 1;
                let mut b2: core::ffi::c_int = *bytes.offset(fresh36 as isize)
                    as core::ffi::c_int;
                let mut vg: core::ffi::c_int = (b1 & 0x3f as core::ffi::c_int)
                    - 32 as core::ffi::c_int;
                px.rgba.r = (px.rgba.r as core::ffi::c_int
                    + (vg - 8 as core::ffi::c_int
                        + (b2 >> 4 as core::ffi::c_int & 0xf as core::ffi::c_int)))
                    as core::ffi::c_uchar;
                px.rgba.g = (px.rgba.g as core::ffi::c_int + vg) as core::ffi::c_uchar;
                px.rgba.b = (px.rgba.b as core::ffi::c_int
                    + (vg - 8 as core::ffi::c_int + (b2 & 0xf as core::ffi::c_int)))
                    as core::ffi::c_uchar;
            } else if b1 & QOI_MASK_2 == QOI_OP_RUN {
                run = b1 & 0x3f as core::ffi::c_int;
            }
            index[(px.rgba.r as core::ffi::c_int * 3 as core::ffi::c_int
                + px.rgba.g as core::ffi::c_int * 5 as core::ffi::c_int
                + px.rgba.b as core::ffi::c_int * 7 as core::ffi::c_int
                + px.rgba.a as core::ffi::c_int * 11 as core::ffi::c_int
                & 64 as core::ffi::c_int - 1 as core::ffi::c_int) as usize] = px;
        }
        *pixels.offset((px_pos + 0 as core::ffi::c_int) as isize) = px.rgba.r;
        *pixels.offset((px_pos + 1 as core::ffi::c_int) as isize) = px.rgba.g;
        *pixels.offset((px_pos + 2 as core::ffi::c_int) as isize) = px.rgba.b;
        if channels == 4 as core::ffi::c_int {
            *pixels.offset((px_pos + 3 as core::ffi::c_int) as isize) = px.rgba.a;
        }
        px_pos += channels;
    }
    return pixels as *mut core::ffi::c_void;
}
pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SEEK_END: core::ffi::c_int = 2 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn qoi_write(
    mut filename: *const core::ffi::c_char,
    mut data: *const core::ffi::c_void,
    mut desc: *const qoi_desc,
) -> core::ffi::c_int {
    let mut f: *mut FILE = fopen(
        filename,
        b"wb\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    let mut size: core::ffi::c_int = 0;
    let mut err: core::ffi::c_int = 0;
    let mut encoded: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    if f.is_null() {
        return 0 as core::ffi::c_int;
    }
    encoded = qoi_encode(data, desc, &mut size);
    if encoded.is_null() {
        fclose(f);
        return 0 as core::ffi::c_int;
    }
    fwrite(encoded, 1 as size_t, size as size_t, f);
    fflush(f);
    err = ferror(f);
    fclose(f);
    free(encoded);
    return if err != 0 { 0 as core::ffi::c_int } else { size };
}
#[no_mangle]
pub unsafe extern "C" fn qoi_read(
    mut filename: *const core::ffi::c_char,
    mut desc: *mut qoi_desc,
    mut channels: core::ffi::c_int,
) -> *mut core::ffi::c_void {
    let mut f: *mut FILE = fopen(
        filename,
        b"rb\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    let mut size: core::ffi::c_int = 0;
    let mut bytes_read: core::ffi::c_int = 0;
    let mut pixels: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut data: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    if f.is_null() {
        return NULL_0;
    }
    fseek(f, 0 as core::ffi::c_long, SEEK_END);
    size = ftell(f) as core::ffi::c_int;
    if size <= 0 as core::ffi::c_int
        || fseek(f, 0 as core::ffi::c_long, SEEK_SET) != 0 as core::ffi::c_int
    {
        fclose(f);
        return NULL_0;
    }
    data = malloc(size as size_t);
    if data.is_null() {
        fclose(f);
        return NULL_0;
    }
    bytes_read = fread(data, 1 as size_t, size as size_t, f) as core::ffi::c_int;
    fclose(f);
    pixels = if bytes_read != size {
        NULL_0
    } else {
        qoi_decode(data, bytes_read, desc, channels)
    };
    free(data);
    return pixels;
}

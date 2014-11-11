// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: country.capnp

#![allow(unused_imports)]
#![allow(dead_code)]

pub mod country {
  use capnp::list::{ToU16};

  #[repr(u16)]
  #[deriving(FromPrimitive)]
  #[deriving(PartialEq)]
  pub enum Reader {
    Unknown = 0,
    A1 = 1,
    A2 = 2,
    O1 = 3,
    Ad = 4,
    Ae = 5,
    Af = 6,
    Ag = 7,
    Ai = 8,
    Al = 9,
    Am = 10,
    Ao = 11,
    Ap = 12,
    Aq = 13,
    Ar = 14,
    As = 15,
    At = 16,
    Au = 17,
    Aw = 18,
    Ax = 19,
    Az = 20,
    Ba = 21,
    Bb = 22,
    Bd = 23,
    Be = 24,
    Bf = 25,
    Bg = 26,
    Bh = 27,
    Bi = 28,
    Bj = 29,
    Bl = 30,
    Bm = 31,
    Bn = 32,
    Bo = 33,
    Bq = 34,
    Br = 35,
    Bs = 36,
    Bt = 37,
    Bv = 38,
    Bw = 39,
    By = 40,
    Bz = 41,
    Ca = 42,
    Cc = 43,
    Cd = 44,
    Cf = 45,
    Cg = 46,
    Ch = 47,
    Ci = 48,
    Ck = 49,
    Cl = 50,
    Cm = 51,
    Cn = 52,
    Co = 53,
    Cr = 54,
    Cu = 55,
    Cv = 56,
    Cw = 57,
    Cx = 58,
    Cy = 59,
    Cz = 60,
    De = 61,
    Dj = 62,
    Dk = 63,
    Dm = 64,
    Do = 65,
    Dz = 66,
    Ec = 67,
    Ee = 68,
    Eg = 69,
    Eh = 70,
    Er = 71,
    Es = 72,
    Et = 73,
    Eu = 74,
    Fi = 75,
    Fj = 76,
    Fk = 77,
    Fm = 78,
    Fo = 79,
    Fr = 80,
    Ga = 81,
    Gb = 82,
    Gd = 83,
    Ge = 84,
    Gf = 85,
    Gg = 86,
    Gh = 87,
    Gi = 88,
    Gl = 89,
    Gm = 90,
    Gn = 91,
    Gp = 92,
    Gq = 93,
    Gr = 94,
    Gs = 95,
    Gt = 96,
    Gu = 97,
    Gw = 98,
    Gy = 99,
    Hk = 100,
    Hm = 101,
    Hn = 102,
    Hr = 103,
    Ht = 104,
    Hu = 105,
    Id = 106,
    Ie = 107,
    Il = 108,
    Im = 109,
    In = 110,
    Io = 111,
    Iq = 112,
    Ir = 113,
    Is = 114,
    It = 115,
    Je = 116,
    Jm = 117,
    Jo = 118,
    Jp = 119,
    Ke = 120,
    Kg = 121,
    Kh = 122,
    Ki = 123,
    Km = 124,
    Kn = 125,
    Kp = 126,
    Kr = 127,
    Kw = 128,
    Ky = 129,
    Kz = 130,
    La = 131,
    Lb = 132,
    Lc = 133,
    Li = 134,
    Lk = 135,
    Lr = 136,
    Ls = 137,
    Lt = 138,
    Lu = 139,
    Lv = 140,
    Ly = 141,
    Ma = 142,
    Mc = 143,
    Md = 144,
    Me = 145,
    Mf = 146,
    Mg = 147,
    Mh = 148,
    Mk = 149,
    Ml = 150,
    Mm = 151,
    Mn = 152,
    Mo = 153,
    Mp = 154,
    Mq = 155,
    Mr = 156,
    Ms = 157,
    Mt = 158,
    Mu = 159,
    Mv = 160,
    Mw = 161,
    Mx = 162,
    My = 163,
    Mz = 164,
    Na = 165,
    Nc = 166,
    Ne = 167,
    Nf = 168,
    Ng = 169,
    Ni = 170,
    Nl = 171,
    No = 172,
    Np = 173,
    Nr = 174,
    Nu = 175,
    Nz = 176,
    Om = 177,
    Pa = 178,
    Pe = 179,
    Pf = 180,
    Pg = 181,
    Ph = 182,
    Pk = 183,
    Pl = 184,
    Pm = 185,
    Pn = 186,
    Pr = 187,
    Ps = 188,
    Pt = 189,
    Pw = 190,
    Py = 191,
    Qa = 192,
    Re = 193,
    Ro = 194,
    Rs = 195,
    Ru = 196,
    Rw = 197,
    Sa = 198,
    Sb = 199,
    Sc = 200,
    Sd = 201,
    Se = 202,
    Sg = 203,
    Sh = 204,
    Si = 205,
    Sj = 206,
    Sk = 207,
    Sl = 208,
    Sm = 209,
    Sn = 210,
    So = 211,
    Sr = 212,
    Ss = 213,
    St = 214,
    Sv = 215,
    Sx = 216,
    Sy = 217,
    Sz = 218,
    Tc = 219,
    Td = 220,
    Tf = 221,
    Tg = 222,
    Th = 223,
    Tj = 224,
    Tk = 225,
    Tl = 226,
    Tm = 227,
    Tn = 228,
    To = 229,
    Tr = 230,
    Tt = 231,
    Tv = 232,
    Tw = 233,
    Tz = 234,
    Ua = 235,
    Ug = 236,
    Um = 237,
    Us = 238,
    Uy = 239,
    Uz = 240,
    Va = 241,
    Vc = 242,
    Ve = 243,
    Vg = 244,
    Vi = 245,
    Vn = 246,
    Vu = 247,
    Wf = 248,
    Ws = 249,
    Xx = 250,
    Ye = 251,
    Yt = 252,
    Za = 253,
    Zm = 254,
    Zw = 255,
    Max = 256,
  }
  impl ToU16 for Reader {
    #[inline]
    fn to_u16(self) -> u16 { self as u16 }
  }
}
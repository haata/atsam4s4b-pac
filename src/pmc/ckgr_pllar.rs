#[doc = "Reader of register CKGR_PLLAR"]
pub type R = crate::R<u32, super::CKGR_PLLAR>;
#[doc = "Writer for register CKGR_PLLAR"]
pub type W = crate::W<u32, super::CKGR_PLLAR>;
#[doc = "Register CKGR_PLLAR `reset()`'s with value 0x3f00"]
impl crate::ResetValue for super::CKGR_PLLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f00
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PLLACOUNT`"]
pub type PLLACOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLACOUNT`"]
pub struct PLLACOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLACOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MULA`"]
pub type MULA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MULA`"]
pub struct MULA_W<'a> {
    w: &'a mut W,
}
impl<'a> MULA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ONE`"]
pub type ONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONE`"]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PLLA Front_End Divider"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&self) -> PLLACOUNT_R {
        PLLACOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&self) -> MULA_R {
        MULA_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLA Front_End Divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&mut self) -> PLLACOUNT_W {
        PLLACOUNT_W { w: self }
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&mut self) -> MULA_W {
        MULA_W { w: self }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
}

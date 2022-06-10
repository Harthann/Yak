%ifndef BOOT_H
%define BOOT_H

; BIOS interrupt call
; https://en.wikipedia.org/wiki/INT_10H

%define BINT_SET_VIDEO		0x00
%define BINT_SET_TEXT		0x01
%define BINT_SET_CURSOR		0x02
%define BINT_GET_CURSOR		0x03
%define BINT_READ_LIPEN		0x04
%define BINT_SELECT_PAGE	0x05
%define BINT_SCROLL_UP		0x06
%define BINT_SCROLL_DOWN	0x07
%define BINT_READA_CHAR		0x08
%define BINT_WRITEA_CHAR	0x09
%define BINT_WRITE_CHAR		0x0a
%define BINT_SET_COLOR		0x0b; bh: 0x0 - background/border | bh: 0x1 - palette
%define BINT_WRITE_GPX		0x0c
%define BINT_READ_GPX		0x0d
%define BINT_TELETYPE		0x0e
%define BINT_GET_VIDEO		0x0f
%define BINT_CHANGE_TXT		0x11
%define BINT_WRITE_STR		0x13

; multiboot
%define MAGIC				0xe85250d6

%endif

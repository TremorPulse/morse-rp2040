MEMORY {
    /* NOTE: Do not try to make the BOOT2 region executable! */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    /* Important: RP2040 flash starts at 0x10000100 after the bootloader */
    /* Begin flash at 0x10000100, after the bootloader */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
    /* SRAM banks can be used for special purposes */
    SRAM4 : ORIGIN = 0x20040000, LENGTH = 4K
    SRAM5 : ORIGIN = 0x20041000, LENGTH = 4K
}

/* cortex-m-rt will place the vector_table at the beginning of FLASH */

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2

    /* Let cortex-m-rt place the vector table and other RT sections */
    /* The order will be: vector_table, .text, .rodata, etc. */
}

 /* USE THE LINKER BELOW TO LINK TRANSMITTER.RS AND RECEIVER.RS  */

/*

MEMORY {
    /* NOTE: Boot2 section needs to be at the start of flash */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x200  /* Increased from 0x100 to 0x200 */
    
    /* Start flash right after the enlarged boot2 section */
    FLASH : ORIGIN = 0x10000200, LENGTH = 2048K - 0x200  /* Adjusted to account for larger BOOT2 */
    
    /* RAM configuration */
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
    SRAM4 : ORIGIN = 0x20040000, LENGTH = 4K
    SRAM5 : ORIGIN = 0x20041000, LENGTH = 4K
}

/* Include the bootloader firmware */
EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* Boot loader section */
    .boot2 ORIGIN(BOOT2) :
    {
        /* This must be first in flash */
        KEEP(*(.boot2));
    } > BOOT2
}

/* Set _stext to after vector_table to fix overlap */
_stext = ORIGIN(FLASH) + 0xc0; /* This leaves plenty of room for vector table */

/* Standard sections that follow */
SECTIONS {
    /* Boot ROM info section - kept for compatibility */
    .boot_info : ALIGN(4)
    {
        KEEP(*(.boot_info));
    } > FLASH
} INSERT AFTER .vector_table;

/* Binary info entries for picotool */
SECTIONS {
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

/*
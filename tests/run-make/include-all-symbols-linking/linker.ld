SEARCH_DIR("=/usr/lib");
SECTIONS
{
    .static : ALIGN(4)
    {
        KEEP(*(.rodata.STATIC));
    }
}

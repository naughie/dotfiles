#!/usr/bin/env perl
$latex = 'uplatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error';
$latex_silent = 'uplatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error';
$xelatex = 'xelatex -no-pdf -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error %O %S';
$lualatex = 'lualatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error %O %S';
$xdvipdfmx = 'xdvipdfmx %O -o %D %S';
$bibtex = 'upbibtex';
$dvipdf = 'dvipdfmx -f classico.map %O -o %D %S';
$makeindex = 'upmendex -c %O -o %D %S';
$max_repeat = 5;
$pdf_mode = 4;
$clean_ext = 'out dvi xdv snm nav synctex.gz';
$preview_mode = 0;
$pdf_previewer = 'start zathura';

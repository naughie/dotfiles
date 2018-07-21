#!/usr/bin/env perl
$latex = 'uplatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error';
$latex_silent = 'uplatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error';
$xelatex = 'xelatex -no-pdf -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error %O %S';
$xdvipdfmx = 'xdvipdfmx %O -o %D %S';
$bibtex = 'upbibtex';
$dvipdf = 'dvipdfmx -f classico.map %O -o %D %S';
$makeindex = 'upmendex %O -o %D %S';
$max_repeat = 5;
$pdf_mode = 3;
$clean_ext = 'out synctex.gz dvi xdv snm nav';
$preview_mode = 1;
$pdf_previewer = 'open -ga /Applications/Skim.app';

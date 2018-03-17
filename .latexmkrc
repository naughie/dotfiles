#!/usr/bin/env perl
$latex = 'uplatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error';
$latex_silent = 'uplatex -synctex=1 -halt-on-error -interaction=nonstopmode -file-line-error';
$bibtex = 'upbibtex';
$dvipdf = 'dvipdfmx -f classico.map %O -o %D %S';
$makeindex = 'mendex %O -o %D %S';
$max_repeat = 5;
$pdf_mode = 3;
$clean_ext = 'out synctex.gz dvi';
@default_files = ('root.tex');
$preview_mode = 1;
$pdf_previewer = 'open -ga /Applications/Skim.app';

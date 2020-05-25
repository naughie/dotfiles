augroup RemoveSpacesAfterZenkakuPunc
  autocmd!
  autocmd InsertLeave *.tex '[,']s/，\s+/，/ge
  autocmd InsertLeave *.tex '[,']s/．\s+/．/ge
  autocmd InsertLeave *.tex '[,']s/\s\+（/（/ge
  autocmd InsertLeave *.tex '[,']s/（\s\+/（/ge
  autocmd InsertLeave *.tex '[,']s/\s\+）/）/ge
  autocmd InsertLeave *.tex '[,']s/）\s\+/）/ge
  autocmd InsertLeave *.tex '[,']s/\s\+\\defterm/\\defterm/ge
  autocmd InsertLeave *.tex '[,']s/　/ /ge
  autocmd BufRead,BufNewFile,InsertLeave,InsertEnter *.tex set cole=0
augroup END

augroup ChangeKeywordsOnTeX
  autocmd!
  autocmd BufNewFile,BufRead *.tex setlocal iskeyword=@,48-57,_,-,:,192-255
augroup END

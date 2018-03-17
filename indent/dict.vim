setlocal shiftwidth=2 softtabstop=2
setlocal indentexpr=GetDictIndent()

function! GetDictIndent() abort
  let plnum = prevnonblank(v:lnum - 1)
  if plnum != v:lnum - 1
    return indent(plnum) - &l:shiftwidth
  elseif getline(plnum) =~? '^\s*-\s\w\+\(\s[a-zA-Z/]\+\)*:\s/.*/\s.\+$'
    return indent(plnum) + &l:shiftwidth
  endif
  return -1
endfunction

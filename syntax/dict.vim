syn match   dictHead "^-\s\w\+:\s\/.*\/\s.*$" contains=dictWord,dictTail
syn match   dictWord "^-\s\w\+" contained contains=dictList
syn match   dictList "^-\s" contained
syn match   dictTail "\/.*\/\s.*$" contained contains=dictPron,dictOpts
syn region  dictPron start="\/" end="\/" contained
syn match   dictOpts "\s\(v\s\[\(T\)]\|abj\|adv\)$" contained contains=dictPOS,dictOpt
syn match   dictPOS "\s\w\+\s\?"ms=s+1 contained
syn region  dictOpt start="\["hs=s+1 end="]"he=e-1 contained

hi link dictList Identifier
hi link dictWord Title
hi link dictPron ErrorMsg
hi link dictPOS Type
hi link dictOpt Type

syn match   dictDesc "^\s\+\d\+\.\s.*$" contains=dictNumHead,dictEx
syn match   dictNumHead "^\s\+\d\+\." contained contains=dictNum
syn match   dictNum "\d\+\." contained
syn region   dictEx matchgroup=dictAst start="\*" end="\*" skip="\\\*" concealends contained

hi link dictNum dictList
hi link dictEx htmlItalic

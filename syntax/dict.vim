syn match   dictHead "^-\s\w\+\(\ssb\/sth\)\?\(\s\w\+\)\?:\s\/.*\/\s.*$" contains=dictWord,dictTail
syn match   dictWord "^-\s\w\+\(\ssb\/sth\)\?\(\s\w\+\)\?" contained contains=dictList,dictSome
syn match   dictList "^-\s" contained
syn match   dictSome "sb/sth" contained
syn match   dictTail "\/.*\/\s.*$" contained contains=dictPron,dictOpts
syn region  dictPron start="\/" end="\/" contained
syn match   dictOpts "\s\(v\(\s\[\(T\|T\salways\s+\sadv/prep\)]\)\?\|adj\(\s\[\(only\sbefore\snoun\)]\)\?\|adv\|n\s\[\(C\(\susually\splural\)\?\|U\)]\|phr\sv\(\s\[\(usually\spassive\)]\)\?\)$" contained contains=dictPOS,dictOpt
syn match   dictPOS "\s\(v\|adj\|adv\|n\|phr\sv\)\s\?"ms=s+1 contained
syn region  dictOpt start="\["hs=s+1 end="]"he=e-1 contained

hi link dictList Identifier
hi link dictPron ErrorMsg
hi link dictPOS Type
hi link dictOpt Type

"hi link dictWord Title
hi dictWord cterm=bold ctermfg=173
hi dictSome cterm=none ctermfg=173

syn match   dictDesc "^\s\+\d\+\.\s.*$" contains=dictDescHead,dictEx
syn match   dictDescHead "^\s\+\d\+\.\s\(\[\(usually\ssingular\|T\|I\|I,T\)]\s\|\(AmE\s\|BrE\s\)\|(.*)\s\)\{,3}" contained contains=dictNumHead,dictDescInfo
syn match   dictDescInfo "\(\[\(usually\ssingular\|T\|I\|I,T\)]\s\|\(AmE\s\|BrE\s\)\|(.*)\s\)" contained contains=dictDescOpt,dictDescRegion,dictDescUsage
syn region  dictDescOpt start="\["hs=s+1 end="]"he=e-1 contained
syn match   dictDescRegion "\(AmE\|BrE\)" contained
syn region  dictDescUsage start="("hs=s+1 end=")"he=e-1 contained
syn match   dictNumHead "^\s\+\d\+\." contained contains=dictNum
syn match   dictNum "\d\+\." contained
syn region  dictEx matchgroup=dictAst start="\*" end="\*" skip="\\\*" concealends contained

hi link dictNum dictList
hi link dictEx htmlItalic
hi link dictDescOpt dictOpt
hi link dictDescRegion Float
hi dictDescUsage cterm=bold

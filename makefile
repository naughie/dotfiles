CONFIGDIR = $(PWD)
XDG_CONFIG_HOME = $(HOME)/.config
NVIMDIR = nvim
TERMINFODIR = terminfo
HOMEDIR = home
VIMRC = vimrc
INITVIM = init.vim
DEIN = dein.toml
LAZY_DEIN = lazy_dein.toml

install:
	make mkdir
	make ln
	make tic

ln:
	ln -s $(CONFIGDIR)/$(NVIMDIR) $(XDG_CONFIG_HOME)/$(NVIMDIR)
	ln -s $(CONFIGDIR)/$(VIMRC) $(XDG_CONFIG_HOME)/$(NVIMDIR)/$(INITVIM)
	ln -s $(XDG_CONFIG_HOME)/$(NVIM)/$(INITVIM) $(HOME)/.$(VIMRC)
	ln -s $(XDG_CONFIG_HOME)/$(NVIM)/$(DEIN) $(HOME)/.$(DEIN)
	ln -s $(XDG_CONFIG_HOME)/$(NVIM)/$(LAZY_DEIN) $(HOME)/.$(LAZY_DEIN)
	ln -s $(CONFIGDIR)/$(TERMINFODIR) $(XDG_CONFIG_HOME)/$(TERMINFODIR)
	ls -A $(CONFIGDIR)/$(HOMEDIR) | xargs -I{} ln -s $(CONFIGDIR)/$(HOMEDIR)/{} $(HOME)/{}

rm:
	rm $(XDG_CONFIG_HOME)/$(NVIMDIR)
	rm $(XDG_CONFIG_HOME)/$(TERMINFODIR)
	ls -A $(CONFIGDIR)/$(HOMEDIR) | xargs -I{} rm $(HOME)/{}

mkdir:
	mkdir -p $(XDG_CONFIG_HOME)

tic:
	ls -A $(XDG_CONFIG_HOME)/$(TERMINFODIR) | xargs -I{} tic $(XDG_CONFIG_HOME)/$(TERMINFODIR)/{}

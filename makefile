CONFIGDIR = $(PWD)
XGD_CONFIG_HOME = $(HOME)/.config
NVIMDIR = nvim
TERMINFODIR = terminfo
HOMEDIR = home
VIMRC = vimrc
INITVIM = init.vim

install:
	make mkdir
	make ln
	make tic

ln:
	ln -s $(CONFIGDIR)/$(NVIMDIR) $(XGD_CONFIG_HOME)/$(NVIMDIR)
	ln -s $(CONFIGDIR)/$(VIMRC) $(XGD_CONFIG_HOME)/$(NVIMDIR)/$(INITVIM)
	ln -s $(CONFIGDIR)/$(TERMINFODIR) $(XGD_CONFIG_HOME)/$(TERMINFODIR)
	ls -A $(CONFIGDIR)/$(HOMEDIR) | xargs -I{} ln -s $(CONFIGDIR)/$(HOMEDIR)/{} $(HOME)/{}

rm:
	rm $(XDG_CONFIG_HOME)/$(NVIMDIR)
	rm $(XDG_CONFIG_HOME)/$(TERMINFODIR)
	ls -A $(CONFIGDIR)/$(HOMEDIR) | xargs -I{} rm $(HOME)/{}

mkdir:
	mkdir -p $(XGD_CONFIG_HOME)

tic:
	ls -A $(XGD_CONFIG_HOME)/$(TERMINFODIR) | xargs -I{} tic $(XGD_CONFIG_HOME)/$(TERMINFODIR)/{}

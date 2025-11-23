CONFIGDIR = $(PWD)
XDG_CONFIG_HOME = $(HOME)/.config
NVIMDIR = nvim
TERMINFODIR = terminfo
PROFILE = shell/profile.sh
FISHDIR = shell/fish
MISCDIR = misc
WEZTERM = wezterm.lua
BINDIR = bin

install:
	make mkdir
	make ln
	make tic

ln:
	ln -s -i $(CONFIGDIR)/$(NVIMDIR) $(XDG_CONFIG_HOME)/nvim
	ln -s -i $(CONFIGDIR)/$(PROFILE) $(HOME)/.profile
	ln -s -i $(CONFIGDIR)/$(FISHDIR) $(XDG_CONFIG_HOME)/fish
	ln -s -i $(CONFIGDIR)/$(WEZTERM) $(HOME)/.wezterm.lua
	ln -s -i $(CONFIGDIR)/$(TERMINFODIR) $(XDG_CONFIG_HOME)/terminfo
	ls -A $(CONFIGDIR)/$(BINDIR) | xargs -I{} ln -s -i $(CONFIGDIR)/$(BINDIR)/{} $(HOME)/$(BINDIR)/{}
	ls -A $(CONFIGDIR)/$(MISCDIR) | xargs -I{} ln -s -i $(CONFIGDIR)/$(MISCDIR)/{} $(HOME)/{}

clean:
	make TARGETFILE=$(XDG_CONFIG_HOME)/nvim -s -i rm
	make TARGETFILE=$(XDG_CONFIG_HOME)/fish -s -i rm
	make TARGETFILE=$(HOME)/.wezterm.lua -s -i rm
	make TARGETFILE=$(XDG_CONFIG_HOME)/terminfo -s -i rm
	ls -A $(CONFIGDIR)/$(BINDIR) | xargs -I{} make TARGETFILE=$(HOME)/$(BINDIR)/{} -s -i rm
	ls -A $(CONFIGDIR)/$(MISCDIR) | xargs -I{} make TARGETFILE=$(HOME)/{} -s -i rm

mkdir:
	mkdir -p $(XDG_CONFIG_HOME)
	mkdir -p $(HOME)/$(BINDIR)

tic:
	ls -A $(XDG_CONFIG_HOME)/terminfo | xargs -I{} tic $(XDG_CONFIG_HOME)/$(TERMINFODIR)/{}

rm:
	test -L $(TARGETFILE) && make TARGETFILE=$(TARGETFILE) -s rmlink
	test -f $(TARGETFILE) && make TARGETFILE=$(TARGETFILE) -s rmfile
	test -d $(TARGETFILE) && make TARGETFILE=$(TARGETFILE) -s rmdir

rmfile:
	printf "** $(TARGETFILE) is a regular file. Running rm $(TARGETFILE) ..."
	rm $(TARGETFILE)
	printf " Done.\n"

rmdir:
	printf "** $(TARGETFILE) is a directory. Running rm -r $(TARGETFILE) ..."
	rm -r $(TARGETFILE)
	printf " Done.\n"

rmlink:
	printf "** $(TARGETFILE) is a symbolic link. Running rm $(TARGETFILE) ..."
	rm $(TARGETFILE)
	printf " Done.\n"

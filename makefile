CONFIGDIR = $(PWD)
XDG_CONFIG_HOME = $(HOME)/.config
NVIMDIR = nvim
TERMINFODIR = terminfo
HOMEDIR = home
WEZTERM = wezterm.lua

install:
	make mkdir
	make ln
	make tic

ln:
	ln -s -i $(CONFIGDIR)/$(NVIMDIR) $(XDG_CONFIG_HOME)/$(NVIMDIR)
	ln -s -i $(CONFIGDIR)/$(WEZTERM) $(HOME)/.$(WEZTERM)
	ln -s -i $(CONFIGDIR)/$(TERMINFODIR) $(XDG_CONFIG_HOME)/$(TERMINFODIR)
	ls -A $(CONFIGDIR)/$(HOMEDIR) | xargs -I{} ln -s -i $(CONFIGDIR)/$(HOMEDIR)/{} $(HOME)/{}

clean:
	make TARGETFILE=$(XDG_CONFIG_HOME)/$(NVIMDIR) -s -i rm
	make TARGETFILE=$(HOME)/.$(WEZTERM) -s -i rm
	make TARGETFILE=$(XDG_CONFIG_HOME)/$(TERMINFODIR) -s -i rm
	ls -A $(CONFIGDIR)/$(HOMEDIR) | xargs -I{} make TARGETFILE=$(HOME)/{} -s -i rm

mkdir:
	mkdir -p $(XDG_CONFIG_HOME)

tic:
	ls -A $(XDG_CONFIG_HOME)/$(TERMINFODIR) | xargs -I{} tic $(XDG_CONFIG_HOME)/$(TERMINFODIR)/{}

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

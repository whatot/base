default: all

# 取得当前子目录深度为1的所有目录名称
exclude_dirs :=
subdirs := $(shell find . -maxdepth 1 -type d)
subdirs := $(basename $(patsubst ./%,%,$(subdirs)))
subdirs := $(filter-out $(exclude_dirs),$(subdirs))

BUILD_DIRS := $(subdirs)
CLEAN_DIRS := $(addprefix _clean_, $(subdirs))

.PHONY: all clean $(subdirs)

all: $(BUILD_DIRS)
$(BUILD_DIRS):
	$(MAKE) -C $@

clean: $(CLEAN_DIRS)
$(CLEAN_DIRS):
	$(MAKE) -C $(patsubst _clean_%,%,$@) clean

# cd src/ && $(MAKE) 等价于 $(MAKE) -C src/

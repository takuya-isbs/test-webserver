TYPES = python-wsgi rust

define maketarget
        $(foreach name,$(TYPES),$(1)-$(name))
endef

TARGET_BUILD = $(call maketarget,build)
TARGET_SHELL = $(call maketarget,shell)
TARGET_TEST = $(call maketarget,test)

# ex. build-python-wsgi
$(TARGET_BUILD): build-%:
	$(eval CONTNAME=$*)
	docker compose build $(CONTNAME)

# ex. shell-python-wsgi
$(TARGET_SHELL): shell-%:
	$(eval CONTNAME=$*)
	docker compose exec $(CONTNAME) bash

# ex. test-python-wsgi
$(TARGET_TEST): test-%:
	$(eval CONTNAME=$*)
	make build-$(CONTNAME)
	make up
	docker compose exec $(CONTNAME) /bin/sh /SRC/test.sh

up:
	docker compose up -d

ps:
	docker compose ps

down:
	docker compose down

build-all:
	docker compose build

test-all:
	for name in $(TYPES); do \
		$(MAKE) test-$${name}; \
	done

example-python-wsgi:
	curl -s -L "http://localhost/python-wsgi/testpath?a=1&a=2&b=3"
	curl -s -L --basic --user "user1:pass1" "http://localhost/python-wsgi/testpath?a=1&a=2&b=3"
	curl -s -L --oauth2-bearer "ACCESSTOKEN" "http://localhost/python-wsgi/testpath?a=1&a=2&b=3"

example-rust:
	curl -s -L "http://localhost/rust/info/123" | jq .

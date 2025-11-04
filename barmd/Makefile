BINARY_NAME = barmd
INSTALL_DIR = /opt/barmd
SERVICE_FILE = systemd/barmd.service
SERVICE_NAME = barmd
USER = barmd
GROUP = barmd

.PHONY: all build install user

all: install

build:
	cargo build --release

user:
	@if ! id -u $(USER) >/dev/null 2>&1; then \
	    echo "create $(USER)"; \
	    sudo useradd --system --no-create-home --shell /usr/sbin/nologin $(USER); \
	else \
	    echo "$(USER) already exists"; \
	fi

install: build user
	sudo systemctl stop $(SERVICE_NAME)

	@echo "create installation directory"
	sudo mkdir -p $(INSTALL_DIR)

	@echo "copy binary"
	sudo cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/
	sudo chown -R $(USER):$(GROUP) $(INSTALL_DIR)
	sudo chmod +x $(INSTALL_DIR)/$(BINARY_NAME)

	@echo "copy service file"
	sudo cp $(SERVICE_FILE) /etc/systemd/system/
	sudo chmod 644 /etc/systemd/system/$(SERVICE_NAME).service

	@echo "reload systemd"
	sudo systemctl daemon-reload

	@echo "start $(SERVICE_NAME)"
	sudo systemctl enable $(SERVICE_NAME)
	sudo systemctl restart $(SERVICE_NAME)

	@echo "$(BINARY_NAME) installed!"

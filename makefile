BOT_NAME=bot

play-self: build-wasm
	bc19run -r ${BOT_NAME}/ -b ${BOT_NAME}/

build-wasm:
	cd ${BOT_NAME}/native && wasm-pack build
ifneq (,$(shell which wasm-opt))
	echo "Optimizing WASM file..."
	wasm-opt ${BOT_NAME}/native/pkg/native_bg.wasm -o ${BOT_NAME}/native/pkg/native_bg.wasm
else
	echo "Install wasm-opt to further optimize the webassembly code"
endif
	./gen/gen_wasm.py ${BOT_NAME}


compile-bot: build-wasm
	bc19compile -d ${BOT_NAME}/ -o ${BOT_NAME}.js -f

upload: compile-bot
	bc19upload -i ${BOT_NAME}.js

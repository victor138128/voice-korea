assets:
	ln -s platform/assets ./

run: assets
	cd platform && make run

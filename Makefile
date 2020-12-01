.PHONY: fmt
fmt:
	find api_server -name '*.rs' -exec rustfmt -l {} +

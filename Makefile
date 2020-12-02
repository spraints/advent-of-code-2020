.PHONY: fmt
fmt:
	find api_server/src -name '*.rs' -exec rustfmt -l {} +

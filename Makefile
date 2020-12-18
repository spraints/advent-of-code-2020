.PHONY: fmt
fmt:
	find api_server/src rustfast/src -name '*.rs' -exec rustfmt -l {} +

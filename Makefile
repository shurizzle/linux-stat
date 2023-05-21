errno:
	TMPFILE="$$(mktemp)" && \
					trap 'rm -f "$$TMPFILE"' EXIT && \
					cd errno-gen && \
					cargo build --release && \
					cargo run --release > "$$TMPFILE" && \
					rustfmt "$$TMPFILE" && \
					cd .. && \
					cp -fp "$$TMPFILE" src/errno/generated.rs && \
					if git diff --exit-code src/errno/generated.rs; then \
						touch src/errno/generated.rs; \
					fi

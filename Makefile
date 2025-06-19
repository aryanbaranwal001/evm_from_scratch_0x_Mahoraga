###################

repo:
	@echo "Enter repo name: "; \
	read msg; \
	gh repo create "$$msg" --public --source=. --remote=origin

upload:
	@git push -u origin main

commit:
	@echo "Enter commit message: "; \
	read msg; \
	git add .; \
	git commit -m "$$msg"

###################
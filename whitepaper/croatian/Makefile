.PHONY: clean

OUTPUT_TITLE = zavrsni

pdf: fer.bst fer.cls zavrsni.bib zavrsni.tex
	exec docker run --rm -i --user="$$(id -u):$$(id -g)" --net=none -v "$$PWD":/data blang/latex bash -c "pdflatex $(OUTPUT_TITLE).tex && bibtex $(OUTPUT_TITLE) && pdflatex $(OUTPUT_TITLE).tex && pdflatex $(OUTPUT_TITLE).tex"
clean:
	rm -v $(OUTPUT_TITLE).pdf *.aux *.bbl *.blg *.log *.toc

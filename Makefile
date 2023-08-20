poem.pdf: *.tex *.bib *.sty algorithms/*.tex
	xelatex poem.tex && \
	bibtex poem && \
	xelatex poem.tex && \
	xelatex poem.tex && \
	rm -rf *.aux *.log *.out;

minimal:
	xelatex poem.tex

clean:
	$(RM)  *.log *.aux \
	*.cfg *.glo *.idx *.toc \
	*.ilg *.ind *.out *.lof \
	*.lot *.bbl *.blg *.gls *.cut *.hd \
	*.dvi *.ps *.thm *.tgz *.zip *.rpi \
	*.d *.fls *.*.make *.fdb_latexmk *.run.xml \
	*.synctex.gz *.bcf
	$(RM) poem.pdf


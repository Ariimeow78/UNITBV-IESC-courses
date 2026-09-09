SRC_DIR = src

.PHONY: run_awk1 run_awk2 run_awk3 run_awk4 run_awk5 \
	run_grep1 run_grep2 run_grep3 run_grep4 run_grep5 \
	run_sed1 run_sed2 run_sed3 run_sed4 run_sed5 prep prep_root

prep:
	./prep_lab07.sh

prep_root:
	sudo ./prep_lab07.sh --with-root

run_awk1:
	sh $(SRC_DIR)/awk_01_current_time.sh

run_awk2:
	sh $(SRC_DIR)/awk_02_cpu_over_10s.sh

run_awk3:
	sh $(SRC_DIR)/awk_03_top10_largest.sh .

run_awk4:
	sh $(SRC_DIR)/awk_04_send_messages.sh studenti.lst mesaj.txt 221

run_awk5:
	sh $(SRC_DIR)/awk_05_owner_diff.sh fixtures/awk5_demo

run_grep1:
	sh $(SRC_DIR)/grep_01_show_text_files.sh .

run_grep2:
	sh $(SRC_DIR)/grep_02_files_with_word.sh shell Lab.org Makefile

run_grep3:
	sh $(SRC_DIR)/grep_03_binary_files.sh .

run_grep4:
	sh $(SRC_DIR)/grep_04_user_servers.sh $$USER

run_grep5:
	sh $(SRC_DIR)/grep_05_users_by_gid.sh 1000

run_sed1:
	sh $(SRC_DIR)/sed_01_delete_words.sh data/sample1.txt este numarul

run_sed2:
	sh $(SRC_DIR)/sed_02_delete_lines_with_text.sh grep data/sample1.txt data/sample2.txt

run_sed3:
	sh $(SRC_DIR)/sed_03_delete_first30_matching.sh text data/sample1.txt data/sample2.txt

run_sed4:
	sh $(SRC_DIR)/sed_04_prefix_lowercase.sh PRE_ data/sample1.txt

run_sed5:
	sh $(SRC_DIR)/sed_05_delete_digits.sh data/sample1.txt data/sample2.txt

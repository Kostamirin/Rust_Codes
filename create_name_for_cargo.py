main_string = input("Enter LeetCode problem name: ")

for i in range(len(main_string) - 1):
    if main_string[i] == " " or main_string[i] == ".":
        main_string = main_string[:i] + "-" + main_string[i+1:]
    if main_string[i-1] == "-" and main_string[i] == "-":
        main_string = main_string[:i] + main_string[i+1:]
main_string = "_" + main_string

print("---NEW NAME---")
print("")
print(main_string)
print("")
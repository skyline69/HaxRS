import os
import re
from bs4 import BeautifulSoup

def main():
    print("\033[90mstarting cleanup...\033[0m")
    # Define the root directory
    root_dir = os.getcwd()

    # Walk through the root directory
    for dir_name, sub_dir_list, file_list in os.walk(root_dir):
        # Extract POST parameters from the PHP file
        post_params = {}
        for file_name in file_list:
            if file_name.endswith(".php"):
                file_path = os.path.join(dir_name, file_name)
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as file:
                    content = file.read()
                    matches = re.findall(r"_POST\['(.*?)'\]", content)
                    for match in matches:
                        if "pass" in match.lower():
                            post_params[match] = 'password'
                        else:
                            post_params[match] = 'username'

        # Update the corresponding name attributes in the HTML file
        for file_name in file_list:
            if file_name in ["login.html", "mobile.html"]:
                file_path = os.path.join(dir_name, file_name)
                try:
                    with open(file_path, 'r+', encoding='utf-8', errors='ignore') as file:
                        soup = BeautifulSoup(file.read(), 'lxml')

                        # Find form tags with action="login.php"
                        forms = soup.find_all('form', action='login.php')
                        
                        # Modify the action attribute
                        for form in forms:
                            form['action'] = 'login'
                            # Also update the name attributes
                            for input_tag in form.find_all('input'):
                                if input_tag.get('type') == 'password':
                                    input_tag['name'] = 'password'
                                elif input_tag.get('type') == 'email' or input_tag.get('type') == 'text':
                                    input_tag['name'] = 'username'
                        
                        # Go back to the start of the file
                        file.seek(0)
                        # Write the new HTML back to the file
                        file.write(str(soup))
                        # Truncate the file in case the new HTML is shorter
                        file.truncate()
                except Exception as e:
                    print(f"Failed to process file {file_path}: {e}")

    # Make a separate pass to delete PHP files
    for dir_name, _, file_list in os.walk(root_dir):
        for file_name in file_list:
            if file_name.endswith('.php'):
                try:
                    os.remove(os.path.join(dir_name, file_name))
                except Exception as e:
                    print(f"Failed to remove file {os.path.join(dir_name, file_name)}: {e}")

if __name__ == "__main__":
    main()
    print("\033[32mDone.\033[0m")
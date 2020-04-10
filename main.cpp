#include <iostream>
#include <string>

int main() {

    std::string userInput = " ";
    int key = 5;
    int choiceToEncryptOrDecrypt;
    char choice = 'E';
    int num = 0;

    do {
        std::cout << "Let me help you ENCRYPT or DECRYPT your messages" << std::endl;
        std::cout << "Press 1 to ENCRYPT\tand 2 to DECRYPT\n" << std::endl;

        std::cout << "Enter your SELECTION: ";
        std::cin >> choiceToEncryptOrDecrypt;
    } while (choiceToEncryptOrDecrypt != 1 && choiceToEncryptOrDecrypt != 2);
    switch (choiceToEncryptOrDecrypt) {
        case 1:

            std::cout << "Enter your message for ENCRYPTION: ";

            std::cin.ignore();
            getline(std::cin, userInput);

            for (char &i : userInput) {
                if (i < 65 || i > 90) {
                    continue;
                }
                if (i == 90) {
                    i = 65;
                }
                i = (i - 65 + key) % 26 + 65;
            }
            std::cout << "ENCRYPTED Message: " << userInput << std::endl;
            break;
        case 2:
            std::cout << "Enter your message for DECRYPTION:";
            std::cin.ignore();
            getline(std::cin, userInput);
            for (char &i:userInput) {
                if (i < 65 || i > 90) {
                    continue;
                }
                if (i - key < 65) {
                    num = abs(i - 65 - key);
                    i = (90 - num + 1);
                } else {
                    i = (i - 65 - key) % 26 + 65;
                }
            }
            std::cout << "DECRYPTED Message: " << userInput << std::endl;
            break;
        default:
            std::cout << "INVALID INPUT" << std::endl;
            break;
    }
    std::cout << "Still want to continue [C for CONTINUE and E to END the program]: ";
    std::cin >> choice;

    while (choice == 'C');
    system("pause");
    std::cout << "Thank You for playing along\n";

    return 0;
}

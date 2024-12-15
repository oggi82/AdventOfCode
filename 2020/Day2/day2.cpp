#include <iostream>
#include <fstream>
#include <cstdint>
#include <vector>
#include <regex>
#include <algorithm>

void read_file()
{
   
}

class PasswordChecker
{
    public:
        PasswordChecker()
        {
        }
        void ReadFile(std::string filename)
        {
            std::string line;
            std::ifstream passwdFile;

            passwdFile.open(filename);
            if(!passwdFile.is_open())
            {
                throw "File not open";
            }
            while(getline(passwdFile, line))
            {
                database.push_back(line);
            }
        }


        
        void print() const
        {
            uint16_t count = 0;
            for(auto & l : database)
            {
                if(checkLinePositions(l))
                {
                    count++;
                } 
                else{
                    std::cout << l << std::endl;
                }               
            }
            std::cout << count << " of " << database.size() << std::endl;
        }

    private:
        bool checkSingleLine(const std::string &line) const
        {
           
            std::smatch sm;
            if(!regex_match(line, sm, regex))
            {
                throw "Actual string is no valid password line.";
            }
            if(sm.size() != 5)
            {
                throw "No so many matches as aspected. I aspect five.";
            }
            auto min = std::atoi(sm[1].str().c_str());
            auto max = std::atoi(sm[2].str().c_str());
            auto value = sm[3].str();
            auto passwort = sm[4].str(); 
            auto n = std::count(passwort.begin(), passwort.end(), *value.c_str());
            if(min <= n && n <= max)
            {
                return true;
            }
            return false;
        }

        bool checkLinePositions(const std::string &line) const
        {
            std::smatch sm;
            if(!regex_match(line, sm, regex))
            {
                throw "Actual string is no valid password line.";
            }
            if(sm.size() != 5)
            {
                throw "No so many matches as aspected. I aspect five.";
            }
            auto pos1 = std::atoi(sm[1].str().c_str());
            auto pos2 = std::atoi(sm[2].str().c_str());
            auto value = sm[3].str();
            auto passwort = sm[4].str();
            int count = (passwort[pos1-1] == *value.c_str()) ? 1 : 0;
            count += (passwort[pos2-1] == *value.c_str()) ? 1 : 0;
            return (count ==1);
        }
        std::regex const regex = std::regex("(\\d+)\\-(\\d+)\\s+([A-Za-z])\\s*:\\s*([A-Za-z]+)");
        std::vector<std::string> database;
};

int main()
{
    PasswordChecker checker;
    checker.ReadFile("input.txt");
    checker.print();

}
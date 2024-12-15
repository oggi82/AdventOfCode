using System;
using System.Collections.Generic;
using System.IO;

namespace day4
{

    class Passport
    {
        private String byr;
        private String iyr;
        private String eyr;
        private String hgt;
        private String hcl;
        private String ecl;
        private String pid;
        private String cid;

        public Passport()
        {
            byr = null;
            iyr = null;
            eyr = null;
            hgt = null;
            hcl = null;
            ecl = null;
            pid = null;
            cid = null;
        }

        public void parsePassport(ref List<string> props)
        {
            foreach(var p in props)
            {
                var key_value = p.Split(':');
                switch(key_value[0])
                {
                    case "byr":
                        this.byr = key_value[1];
                        break;
                    case "iyr":
                        this.iyr = key_value[1];
                        break;
                    case "eyr":
                        this.eyr = key_value[1];
                        break;
                    case "hgt":
                        this.hgt = key_value[1];
                        break;
                    case "hcl":
                        this.hcl = key_value[1];
                        break;
                    case "ecl":
                        this.ecl = key_value[1];
                        break;
                    case "pid":
                        this.pid = key_value[1];
                        break;
                    case "cid":
                        this.cid = key_value[1];
                        break;
                    default:
                        throw new Exception("unknown property");
                }
            }
            props.Clear();
        }
        public bool isValid()
        {
            if(this.byr != null 
            && this.iyr != null 
            && this.eyr != null
            && this.hgt != null 
            && this.hcl != null 
            && this.ecl != null
            && this.pid != null )
                return true;
            return false;

        }
    };

    class Program
    {
        static void Main(string[] args)
        {
            using var file = new StreamReader(@"input.txt");
            string? item;
            List<string> values = new List<string>();
            List<Passport> dict = new List<Passport>();
            UInt16 Valid = 0;

            while((item = file.ReadLine()) != null)
            {
                if(string.IsNullOrEmpty(item))
                {
                    var _obj = new Passport();
                    _obj.parsePassport(ref values);
                    dict.Add(_obj);
                }
                else
                {
                    values.AddRange(item.Split(' '));
                }       
            }

            var obj = new Passport();
            obj.parsePassport(ref values);
            dict.Add(obj);



            Console.WriteLine(dict.Count);
            foreach(var i in dict)
            {
                if(i.isValid())
                {
                    Valid++;
                }
            }
            Console.WriteLine(Valid);
        }

    }
}

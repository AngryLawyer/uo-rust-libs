fn main() {
    let path = ~"../uo-aos/";
    let skills: ~[skills::skill] = skills::load_skills(path);

    for skills.each |skill| {
        io::println(str::from_bytes(skill.name)); 
    }
}

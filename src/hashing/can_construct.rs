use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }
    let mut counts = HashMap::new();
    magazine.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });
    ransom_note.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) -= 1;
    });
    counts.iter().all(|(_, &count)| count >= 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }

    #[test]
    fn example_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }

    #[test]
    fn example_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        assert_eq!(can_construct(ransom_note, magazine), true);
    }

    #[test]
    fn example_4() {
        let ransom_note = "ugifcocbiuwfynpgtwpgfvzhhnalvgubmjqhzegktuxucbmqtgkoopylqiwvwuwrzruzjgtppywtqisdkmibdpgtismffhrrmolmawlgltdmexoatiuyhlwmuxhlrfgqntzshwpoxscqawenpsmwhfqrfldcprfoluoshozfatbcflxeztsmtvqkpnpmshbzdirurkrcrzwpaadmrvpvsqojpnbikvypozubxqhsgwaumbktesbknchnfgxsdnozyixqbbscjeyxnopmxbsjuelauvstiuoxzdfszldvtxqxppumiyxfqjcvzouquaweriawheiabliwrubaprfpzdrxmvnzgypigyscpirasmxgemhrxuecigmrapaywipmucsrhxkdrgnucvmnboevyfkmqlhscturow".to_string();
        let magazine = "fmjiooebgjatqjttfftvobormhubvgxpiobekkxujgvaktcewbvkdnqwqmeeaogpcgaliilvimtsqtlfnhtvvipwsesosgluwtpearpvrrlpjgppqgegejsruiwbnppnlonpsetvfverxhmtihrfcbspupgmhrqniosmosamujoxqdepvjrcnhgqmkuhrlbrwmwtldfjnnkjnvpfonuvpewusqxstgxcfikmcwcgkipootlemscowhstxcaefreafjfxtuqcbdvasncmijmvdjlwcqopxnxdwqafwugelnuwledeajbkvjilvbjkarimpesekhmoixfapihomeswgirtovrgjgarjglnjmshirxphafbghvxuwpxggpbkewemvtrrmjxscdjlfvnfuepqkjffkkmdcstaktwlwawxcpxdgxsidxmcnhhlrtpmqfafucwxdieasfjgueridmakgcehmqtehwbkvahedxscsniwpdannvbpobtskdgmpfijknlunawajfcrgvigbkrlmuonuubmaluqfsjvsajjixbqjqrvwbhebumwsjvmqovawxjqbrumpdttswmqmutdvqvvtujmkbdauckvdcfknhsrwxxaucfqhabuinedmrocivwmhtwlkvfuqjmnouvkquijisidwqoekheegfvxxvdcngjxrjtlxfcgufmcjvtxrjuhipeoenouagigobxjfcpblvirfjvtfbhdmrjpqpvsxhijumbfjehswteajcmqhcdbstolecqlktajmtrhmjddnoawocbohrjqlgckhhhrkbcwgvpxolrlhwuvebsumijspknesmsspcvibsemoccjndalknqlxguvwvhbeujhavjpflwpuexgwwmbaqmlqdilwbaixemaokwsdppqjtgkdgixhkmmoixssvatqguvqjuctwfpvpjulmuuprkobuhkasxabjxvhltaejkuxoxopojcxxnasxcnqwnqsngktxdbhpuxacxgpgfxieeubcgkulklslskdwvimqvpatdvkukejojafuvqbrpqkcbwpohesnmvgpsdqoordplnhnoacwsabsvwjbcujvrrcimqcolvnwckgfbkuuvuqmfjohxrkamlnfrmaexcqeordnwwclgsxpibbgetfwntxpbocgeguaxavmnlkbpkvdamhqlsriuqhempogbsicviftokeiowlreddnlfxvblgpcxhtgovkijdbbtjavfihmwtldogomedftgdwlkkbjlwgogjplgfqimhevlhcntxqaukoqoqoowxkntlxxkbrihahqvsrmikphqvdfrgjarngpkaptrfmufovumglkcdcjeefsmndqnrjccpqwcaiuhifbptjftksxownppntulkdqujrapdfqfxhhqssxbjophmsusfrgarnxawkjwgeqhsdgvlggvvodgwfcvvgexwgrofdsiquvuscopwoewjlkndexhgripagmggvbfswngxlpgrjxwcmtcdbmrqsrhhgchkhxntaklapsshhrdljaghmstcspinnctbkuhuukevmfcwqjgcsqjhtxnnqrsxttisubvusprxurqwnpxxatdkifxbmrnirhxghmnuixbwppcigtwusjmrddqvowawffxntxqeauojlfrebmdwrkttsqsjpiebknsvgqwdrvgbkvebqnnpeorbpwlogqssfhorkbgkukkomvnaktvhtfwkbmeumuqridupxasqacshphkippegghkprknspngnckmatxiwfhnevkvkqjhtwvvqganrsdogtswdavjtuifwfwuwmjjgknwuaowbkpwoemsbvcvalvqfqlpteqjqsqpuhewrbxacurhbtwmuexpanjajomgbhhwpmswbonkndhwubjvixmcamivcegbaqjmwoigsrhffchxrhdcmdfjmnaitohcgfvaaohnmthqkakkrfmrhaslnbbhxkwrkxvuoknvollvfnxpeemtkgmctplllsknnfxwhetmjbnkosnvjtlpeojmniwtswvxopfpmeklmfeqmgovbscpwxdktstjcuwioowhvibkqaqxirbptkhbpbiegaqllgngvbkgjsxqmrrhbuglqcmoincrgppswdjjsqrejcohbrxpqofrhspwbsqlkrgotemlfllqewnnppkrvxgbrlsbrijatuphrifkqtiltillftkdhpjkwedxihhrrdanjhfiihivgxkbenxwgxgffnexiibgltthtjslaamoemjxtwbslgfkgxulwchwjwhwnpvugmswvwiflnbwwfojtbtbnwrteumskqgskvducfhklkvmgksjjlqbtlpluljtimvxvdabdbohewgkuxxedovvarbpifibqutbhepwhtnmvqfbhugqhijuvqhhbrqtjxojpkcapsvjarqbqpqgneefkjsdoqskdnqxgnxkkwewcprbpclggpxbolqsaamwqphdttqlbidsnuhltnbpclaacpsoxdxhgmfmxgukmxdgrxrnngblqxacvljvcrlvvejebvsacmkcnjbamhkibdhgajtijbqaoalrsjufdvswkbwejxgxeioosjkpnlhpipkecjrjjmrcipqbdxmqkflqurfjghwlnrqnqnjdrqjnwtpwtgedssivnltqtwpecgudvgenwsrjuodfiuohnqdfpoocvhicrxoqakcvfjqedxuulbkgcgghhlnqesdekdockldurnapthceeoigtuanhrqshxpmvlnkkrqkqhiktnovccaindinxqphgxmqborqbdphswaemrowwgttcsqapicjscjrwedegjrcrcqpgdljpqprkrxwvcjglwihfvlvstpmktqrxuweolnamqfxevupqfarwccunchfxcrtkdfasmwsrakcscducgbrakgrwndnawjfnakvenifrigaiqgnicdfenbegujmnxjormlclvnktrbjfpojdhworurfuqtsnwravocjrmdixkdvrpbkdhleaoiujaqwackjcooalserxlnegbjwpcahqdfifmxrpmkspuoqdaptamnobemgxpggkmqcpmctkkllvbubrgmhfgucnimkmogcgxikhbquaodetfxduclvtalriwxjopgedxpigsbtkxmmswhnvwlfvkitsvhfudisjbeeuqnchgnmswujerogjfvquntepjkukhpnqkmxaswecmwnqeejxiqawodxpboetaiudiwlutgudawkbndpqdtkprwwtetahvhjekuwmchmulsqqpuvuhvxqvoadqibmuvmubmhifiuoedgfppwssoxadspiaxhchavtuhqppbtqubohxujlxosqskjsrpscwrditgpknlklbnghoefgleiqspxqnasuhradhxnqjebovnpvuvdmasaptprdtcghfgcnxeckcnacidjacbdnfquvgtpkefamvcswtdgxqoxxiouixwnalesecliwobgcilxgmoiieinmhfxwtorfahtbhigpuorsxtttresmvbtkuklkhqgvgrgwgqhmlnidsjeasukmdkjqamadsloqfcndovuffevwpmldiarixmphklwcejpnmdgaxfflbewtaoieshhxiwnxkxvwfbsfdstekcboxoujuwtuabdswxlpbivqwdwhsrkpodkfqahiuuelkwpisrahngnxwpnmswdctpojduxkhikarojsldfqrbgugnveclifsqwkaruuvkwemrkirvckkbipvnxwshxhsnsxqgvrosrouqkrvxhukwdplhltkbsjtgbisntxdjixhjqvvrthiahfilrstlxcflfowfwodvgxpvovmdiseqdinsrmmuhtrmurmgwhwxcvufsjhfegpeibbscqgurewbxavalkvidcmxvaipiliwohufnhadxuvkweqdeohgknrvhtgrmapevthbcakjxduijmgxsmfthwlfqjrlpwfqvnjqreewkkwjpbguvdnnkxqjpdcgtmravecqklhcqjaokdacfmbokocawjqqgtdwgqdleesgrxeaarmsswifblabdsttbcebrldqqagimnrfkddstcdrfhdohwuvuccjlcsxwobrldhxqcedsabmhscokhcqxsiohijxgxvceiofbgxgvxkfgtnbtcvsbttamdgsikcbimxgpgfbvqpemsvxusgftnfhjduxmqgahqcualvocswraraqdnmifpnthjiwrjlnimvgfvpttwcubfiebththkemgfeximjcnpcuotukcdavqrfugviwrnqcowxtbjowmkiocndcvksnvdogbsvjdjdopubiecmrouiuduetnchshttlumjuennoirlximstlwropaxvcupnismuvirpfkutkserxsbelxlmhqvqtathoisxfrhcohcgfcumxcquvigolfvmcncgcueommsvpvhfbpecbehlkjtutoirvmeeupshsecriubkojdrfjwrdbjhoefpnmcxminughquitsudrrtsrggnrifwxuwkglrbamekflpadsdkhposlwkdqjijgfwejhafopoexdlqtpwjcmetpwnmptoabbqcwhgwlwmnnrgqudxbdauophihnoblrpkvfssfhsfewmcqgnvrjmotjajnldxrgxfwnqukbsjdlewaawqtfxckfcswrjcepskchjmechqilcbpsnafhjasutjcoarkfhqmkwsvrqkvpvngncnagkqnfvirtbueawgdxwglhvhaonrfjfxektcbbpgkxtscxiwonkipdgqjjcnenhxjmdwpassgdonlkuhwfiqbgutsqvrkkhdpmtuhrngnwcurkcbdwspbpobscnfjpnenrjvqnqjpbaxkpstnradwxetiadrxnktumphldmdulxppavxmgrvjwhxnronnmnweuntrehmuvicigcmxmfoklwhjworllwxvsjkaaumghbbvickieemjhbreglwilgexgnsrwqdlamefhecfnprgbrhncoswtuhrpjflgvrdrrustacgauersnavjggtdahngdkijkpwroxctglfmwmbbftvnldiftmbcnjgxhtlbnhophnbkslppccdtoafpmfgewjwpfweqokoasjjskqtekupvgmcjluldpuaclbhppsrwpvwtftfpqrtllrouqqkixruqrknwcgbiptiweenjrbvuogsqujjsgariqdwupvehbrxswtcjpuojosoewakcejqsxdbrejdlvbkalivartpclpcipltisekhoraxtbolmslhldofievwvuwgvdelpbvhwhjtdfmxvdaapufrkqdnnbsedqhemrjbwmfivnpiqwstwekurpxikoncotccpcqggiumxjngfkbpjjefncdpntirishglfafgtvpurxinpeeounumhfbvqtqixuxfdkxhvksbvnhpnvdpwdkaighkwgagqrwpcwlagbvighsumsuuvpxrtnautrbdpasamescodhakestmuwdlaknkunuqdsgcavaaeumnirrkwxuwdcwhimwubobgvehdamxfnqolqcextqpqximivndjvkltxwgjrjwvbhqbllqhefndppwaqevaiblbrjeulenhoqtnilfebhvqfhlwjbltsfigdvieweslfuoaadefjoxvxbtsqnvjdtkmnjljjshjfkjldtmrgrmegnxmnitccokcgfetwwtjqdfebcvkgjdlbkfmpbogqmpppslclhxtexjfkiixcwumsbicqbfoffbnaaqwrvrxcnfvnhdikxavpjvgopbajkitjwuxpruvcocmtmrnfmgaktoqbujfetfrbfgghtnfurhbespwvgwejhsrlqnfnpphkmgrxswxaxmwixplqvrreurxtsnbwwpnkufbbxwevpbshlimhifhfcutqskxncmmsfkxlsnsmcpvaxsbkavgdssotstrtqgsswgrgkutnhvpflwsruoonudwmhivplvqbihbrpkfapmqeodilvcxmpandesntrcorkhwocxcfckbwqsbccarauikphpifcunaevhtdujdxkqsdrxmnavattusisowpdtpikwjjqrthrgkncqkddbglkvsqqogaqcgmfcqftmvjrduwnwegagameuaodcnecivaodawvrjgbqrpqprwhfxprbuucbdurptffbafxixvrtrhfndkjedujmfhgthxwgbtdmsrijhpcgeatpidnfmkvnnjwisoifxbpeulreaudisxlnbrlxcceukckjuahtnhsbhdbjttmaswuwokmrulprnssmnotklhtudiqqamnqgvoeojxntwfmwqhcnafbjgstxdwshihheecgluuspdwnueqdlnuaohppndmfxxkvejwmdeeiijsnrpniaobitidjtrrccrmsqhehpqfhoqcsljxpvkdhqrvxcensoexopemvxbshacfgljvkhccovtftkvjtmdcriqnxtgiipedxjqxhbnwvsgutcmuxuuwlunpfcqbsmbnhvnwikeinwjrjmawevxjedqkqwounmwkuobrerphmkfmewrqebwnmvuhwnbjrsokigtithvtdobvtjjrgwgsujwfrqjrxnxnkdmmumovlkgnqnvrknkwcrrmxhflnmjucoutjidutgemccaqpgxuptlnsjohninpaxcqbluikkkabxqpxrareamxratgfbdeefgimeqscvqdfwfgjntgaavkdkfphxsvoscnkeqbcihewfadveqeckoocadsrjdbgmtwelmfsdxlvvjquuqtuiguvcganrvqmmmlartokohbiwhumwlrnaoeuhgakfbticdnponnkiafixkxgcqqutnuxjkkawxrcwqlrbljkdrancucchqxtjugwwlpxewldxugplntsndiqvkebxcxbklrrxioxwghbwxupmkjchwxhbxfjqpxwfrfqmshiwrxgeqhdmlgcbjveorwudpxsdolunasssudagaxblxpsceawtaoiibxfutsdjdavdcjrhgkljpwlercesvfxwvqqaitossibralpwtcqntctffsabimhhebrpmewogkwmjbmcwoaqkdmglbjbassbglbjipjoxpvltbheihlkfcehjwxcwhxvcebvemjcugeawviebualtgfipbkmrugtrgxuhbvrwaurtupkafbcmhfqolcpkahttjnojimfelgpkpmwwknjcowaospcjlceodrtdqiuhbjbsvhhkmfotbhrwrrovvphvoflvsaxwrurewqwncxchlaigfqdghwnwkxfwvuokmbvqkplwvenxbbaiiernihucfrsxjurtjxxekuowuehdvcabrvxkbkuqbcshgixqqvfdvppmpqpsolpmamheqahjvcnpvirscnptdrvcwwwmvuwpoxxexxtekbtxvuhqwopfvicfshulqnfkkaqjwibsxxlgpmblmalikfuxceresomwevkmterkmmxmkaxkedfesreiwufciqttniffastvbankgjjwhqtlgtjhpoplmrrieiqdnbxlhtwadnwwenwdfsnntejhnkxwrmmpilulfnsvcgpdmtnmhagkfnbhduwengbatxmwdhaskcunnskxfunncrilpvdlwvxpnundkxjhddftjtldookoepokahrshbotxeidbhwcwpjwqmbcsvvwcjknwrhcfkovigmvanvgcicnqpejblwffootjkmdcawuqcdsilwrxngjtcusmpecmpivkejefdhrhatcqplungwrjpbvcsbopcjcrwwoxljeoaneadkoncsoavkceclqgqblmilawqeeutrqjovqglsaktqqjqkrkrpqvkxdulassphqcqkjrqvwodlhreabkovilmuwbkmnofprtqskvrejnetqmhupgnrmpwaktojgkfawvaeksxpiwoallpfdxpmdxauxmqhmoqtfgpccaagebjjpfnmpmnoovcaumxwijmiwjvccqfrmofpnccjifgxmrndjrsgbsdpkdlnjbgqrockelfmbdjogkpxindojmeirwptidmkkqbsfxfupvivuircwaranegckbkuwxvaanahxgulvtkkftmnoalshutwommheudstlqadrrhbwhgrewqohlfhmcvmawgbseicekpjgtrduojrkrdoeqimslmvmoxgskrgamkaqmjeijkfnenxqsvpaowqigdlduxgvisamdomhhlefghjcrsindkgnucpnjatvupleakctrchdjslsfaoskposkttrdvuakiwxlthdnagpmtniwpjvtoshwagobosmaxeovktpufaphnjvfhcqwhxvshbudlvlisdpupuagqnvspuqaajldkhlvjddjdmebsturbfnelgavabrqjnvdokjhgfelhialpfvusepeuljqrojgdxgmnphiwowbhawgqbiwqcexelodsdpqohwpfpwaahcldfgspgpvmwgwlxrpdrdakegfxkfhgiwclwaxkvkdwqnqbxckeaguxofelpfnoqnehmnncfhowkhmvbivmppweglwiwvkjaekgomephiatkoujaljepgvdmjkijfwohffjfgvrkvahqrptslefhejsxgpbfvcxopdirrohpewqfrnuqriuqqjsacxmmoucesgljpasghbxjvwtjudurkxsovrrohwqsonrgiedwvtetnbswmhetckukavadqobqupmhkiapxlwtbrpwbldojkwlstissgvnuwsvpwrterhlktukmgkboircudtdmeururmpxnapmvnapqhbvkbnskvpfnppwkvwoikgimdoxabvrqjucwartrcbvbvgbhrjbdukuvtowidqwtblfkowunijuifxttbnocrckhahqwljbcjnvgvtkpogdawthfcamfcxlcksvkqbhtxccctrxfnrrksjshxktupqgobpgndqkimunhjakrgjwxhcwapmdxawdlefcfnfjpaivdhhtjoplvogkabgnwvacrfvjedrukdjlhmwdhatpghuxqohqkoheqcgmiadtxufntchqblesqrudgwiurkkaiejcldmhqeugaphfrwcfvgxatbakfskprsfluddqmmxteurpwshxeuvwhgarmgphmodqsrbmlkgfrbcqapiinevedpxrqwilxgtqhfovfwivvgvppfiqiwikncwlhbgaetetsprkssolscqndlfimsvixifotbqajfhblptucuxvholrukxptxfhxrtctuxjimddkedcnwuvmdadcthgvgiopeiaplcwxxrqhhcelomcawedvenuaannwdwhgwsuqqqjsahkueanvkssvnqbflfmlespsnpvkvbnmpdvicavsgpptaafephuaahqkpijfxlelmveaxdkhrwsvdxrfwmeuhddovgwjwhwigghskkmnuvueqkmbwlvpawxhhiwncvmehgxaeuucxmubveoobvwbnbvdvhqtkmfvfetinkrhsuccstiwvrxsueltpsfdhfbnjjlmmtmevvprbxrpkaqkwgdhfkltrhrkseistitchokvrjbtckkbdhfqowemqlmwsoreijixbqtvjrjbloeidbxudxaomakahhgbevdmmkttsoaihfxnwklfufwsmxuxlvirpucsffrwubgkuojjdkjjjpowcaivednubphahifwxjmolcentspcbwalesbjcxqskwcnjuxdkxbudqihgrphgrnklomdxsotrrrlcbivhdfuoblbujrxwjubhdttrljhhgobtghmmeigsfhbfpducvncpkfjktageqqhmefacktcideranxvkoquektfabdqafqvncjkaonaqtaqwducwuwxhulqtgdvtfvdfrtiqqgicxfopqdlfoxkgxxqncxgmbpbwqddrhhdguktlfcaqnbxkepwfuqrfrgpgasekephlfiwssurxqwfsdstkmmauedvgrpjmghwciojtwogkaneqdanflvjctvlgkkupgnwlvmeqrkqjuhesfjwgjdwoofbntmtkmdjoncfxnxfensnjawunvunsljivweqppbntqbnrhnwccqivvvrbqcaundwuikkuanejvauvsnuuujlodbmucewvcdvdugbsornreqsvtgvktupdthumstpfdplcpjmrkupmxhqhnqlegbvfvjtcqodmhbkbulvjdpehuhengmfjwiteqgignnkkpjqhjcdrcbtflqafwjclkkdivpdxjamwcnxngawlwkmvvacxghmilwtjllmhdqrrrpguvuwvakhwrbevwhsriufxdklummbldgvjlgeaqcerqcfvnxeokfqjmuqplvmmjdehcpounjuwkeubmccxjhdapktarjcudftvsoviaifvwothkfedmfiemdfwwcesaekstekwifaffijqicsdumonkiuljaqaairpmpluwkoiglwtoiuhagntbxsnkcqmvsqunrbtjpvmcfksthcxrkfnccnfdisxfrpjpcvdssgsrjndhqmlobgevrodbkqqawxokawlqfkskmphdkltqjijbkopxiarxxocvndvnqiuvqaopwngwvdbxxurtduibgvwriopgrphdmjiccenrkippphqfddwhtftnkwcvkjemiimfqoabsncqtbnwovljuuocctgslndpnwmaanxfmdjkcjanocxgdekpqnleklunsmgkqpjoqdvwehbmkscxieovnuqeblxgxadtkoidhtgusdhatvdtrtrmhqxkxvtrvmtmfrsvafmdknmextdlidmtbjwffjcwlgvfkiqttqoehvgfjmspedpdaxwvjxcoralvqskobtowotoooiqwgdslospdvpubqbdnpjqwiqkkjpufapvandumscahjbiopjguimjktodixkqsociortpveqlqegvogqmljlotpnbxqkbgipsislbxvkgxwenqngnsmxxawflpniijxaefjddceskuxmovflaabgecpwtjnvockldoflnvrgpoixbevponxjnqdaxvqpqsjkcshugqvifvilggwsqsoisdbtaspqrqftikveiehplkxkutpethwukkrxwjbpafnqmobubbxcsfwtncivklalgwdvbcoluiraxnlvkxwdcntnxmrouptxgxqtiehookjtjhrfdukjtdjstatfacpwkmxcfxtpwlwrswvgmdgufwfjbhvlhfvsbwqtwmsoukicbhhxespuahrsjqxbrqucfuemututiqfwavnhujprlnmvsukfgcefmhspgdswbuxvvvbinsvdhjhcgtfboenfhgskovlhqxfprkcrfkssmtbrtuiuanijgsvhkiqwfixumsrsudmocccccievfwvrrdsvenxliqoegnnlmnnqapcwsgaovwcnxqjmculpjwrigfomtivwijkdvwmoawbvjkseumbihljwfhuvstccqrjkjcafwvbrpiovufdhexuitvigjggsrwxmiandpekrjtmhjjqvojmeqxbqakjatjvpiiciklnwxknksnqqgkauenpjpxbcgkcvrliuabbojoigquesdrrxpwtkvuisrqgufjwhjhpkoovltttkphgmqsueekoxvdcgkbmkfmntxlerkvnhitxfaktvcqjadnfkcmctljfxmnwrxistawfjpgwxhhtpilxqljjtvvhqntcjaetnnixqnnpivgllpaaqgbitjjbvboqlocpfxafsjqnclosupkjsclpjqvffsnbnrkxtqxppforuixhshjjojihhjqkkgjujjtcxwuwppqcvnicfqobebqhuwdrtwrbngoirlqbsavixkklfcraedchhgnjaswrlhcjrfkbiiourpeltnbwoxgpcaxkrsadkfqohvlmkwtndlhrfrotsvnimniegfgpcqigusdeguldficfmnkqanxkvnjaheafcajcgwrbfqxwhmoijkkubgmtvxjfmhiwevlaohfrqlwprqdrsijxlbsvenrmngdrhgicfoagfnjnmmiirxfkqoupsnbekjmhmufablcqgwhagpvwehweovakxiqvmibhrpnqegcnkvrrqdtsfxffpgwelgxhrkkjxhfumorhaeoplwsmdjsosqqhopftdaangeqddvxosbhtvrbngluggndljltaqsdewgugijlqhnudambmoouffdsxcjiosisnqubellnxpaubchcbsrruwcslcsvohrwikpjfudoscrqunbtumwurbuwxsgchrpwgwxbctagsasxvategttvxwqmeqnorpatgcmtrqeevbrctdogwqjltnvuqwwrurnmianhvtnjwvhbovqrfrqgxguxgdtfrowknpufdmuecuutxibubpihcbnndxkxxadvehhtfdbgtppquldinkroehqfjjmkxwtunxpesalpimkrioevodccwwmixtruaihpibeocuuormdoajnkwaeehplvwernsxhqgufswnklxilownwxmdplittvvedonbduwampvvwajgcsjixnislihqvietrpcxdgbnhvhsakxaibbdaudpqdwibnljrpirxuwlhonwiekfemxhqdtavtnrxethuppqowvlmqgxotrmalkcjtlxejsvggttrgasbvgtotkpbsexknkvjtadtmptxteocncjqjsuuqsnshlvlcibimuutfdtscurvdvddbqhipvjbshwdetvafoctdpexmnstjscrgqkapttwnpcjqtiahpnvxciwrhkupanuplhniudqfejhrruxkgfhaewghaciuptkfamgebjrunltumqoflcthfgjbkjhaqwehhdfabnpccqgueenjoehmojvvvoqeknsgrlihnhthdiaxcaevildgofleptnivaucftihepfbbeqnmuckqajgoddfsheurxrcdnfaofvttsqivajxojxdudejfkvgpcswmixttskdbdgfsehntphgkoahbecrgjxawmgiuufseadonqvdwlxgqcnavvclpmqfmjjganhhdxhtvuttxdhmebelbvevamvqlhsghlwtockesqakxjraajnjfejwmxwhnohhcasvgqlinvgsvemreuxcscjwqgscrorcicngkqepvjldrdxpgiefpxmpgscchipojtffkutccgaemidtaxkleubxvjwpqctmfsllhtbjnkvgfsqfrikdspkovnoqdntuvppiavvdfxkjurguklmoqjnvhfjgasafrkjtnsjrklournejsaadplldrqbaqxspxlregclsvqchvodmrpftfuujgampkwrubsofenvpnnxthemeaavoulxhtelkwgfjoxrtwlnhkcliqhvjvloddcwekeocnofjwskbfmdlfxwfhiofcabsdihjsanhwqsskdbfhjjtemkdmfjfxpfndmtnmaxijluerkwxwqggcrvkpxmllbejdisbhmfvphlqmqaiksjvmwqqjpjxgeuretufdmpawmxgbdbijqtxmldardorajplefiqqqforshnsmpwfqmfrioihnmacjdxgoxshaxkvrmohmtdomewlnrbedkmdtaghgbnxvfcmbwiwkfwcdkltllcppcwdoaaroogqvgwvcnmjfeqlmhlaaoeaqcotesudewilslhqujraxrajrepkoawxwllbensnbrqxkunoajirxfpsjxhctjbvingafcmagormhjpgepbgfwvqeqkpjlukqiwfbbitbwrtproacjdgqujscbrrxfimftushjogmritmwrfaawscjujtalbeovnpipgxmpcvxhevwbequerstvdhgrgchkegnikjicgoqoxkcvnquctjendwmdiqiabldxvvorcvtelkkwjcklivclwqxaxgjgvustxegbxmrcahouxfefrhoiwosamurfxsjnaeameckhgqnwxblivhplkwdmwksuiocmjwukrxguwvaesoxfuohncjmegisegotrjcbkwdfjhppqhmxmhkuwxatplwcuttvlabanuggfkesxolvufmxfqjjofucgpoqlxbblkidqvnbuxdhwovqlvgnaedrhqolrfmmlbrklscevluwrttxhhwrorbbjqkimsrbfabuojuckrwkgdqdcfitadxqewunmxhplxibqaruurjtugtwmxgmvsgphddtdlaqcgpakjtkdmgimfewunpmfpuwdwhkgwlsmgucmuiewrtsoxsekaltcrdfixqcqaigbobqnhkjorjkjmpqhpcjrqctfhochshlbwmifmqdcsbvwmkrllevnfajcmjmtdalthxcfdbqtgswuqljvlbamxhiijbgvulmhiaqncprimleupkwwxjkwwowpdgjnuipgrjqedjocnwgvkabfhutucntoeinkdpuilbedvhpjisnqitctuttwuebwlxjerilmmikjrrtunxbubmqqnerdllooaexvpttlxftpfxdtvuckvlfdcaxdakdpmnfseromrcoemjhixtqsvbhmmalasjabouewqqcamwnstxxiddstqmhmkkgxjjflohmxesihxjggmslwjekgwgnleahtnqkgvamnhfialmksgilacxosqvjjdbqshpgjsowjjtclgnrwbsgofchqfttmwupegpdfniadldnjlfdrobfrlwuhnpcpflfcsghbpadkemskhuheedijslpabjxibfsumwcwgvphmirvmtxotspvmsgdpikhrcwbdfodvhmbuwcdbajqunjojbsxkwlnfsbkhfkjbosbkwnlgfosgtqthktnhskcffsgjffamsihrarwuqwlrwcfqdjuquofslkoiuqxvthkgeqectiegfadmotfueqplujbmelxvbhsdsqeaewmdqsgrkdxmdqprqxqveohkuaixkcpwvoelxaobghlgxgwsaetmuiawfjmkvwevfbtlsjctmqaejsuashbqmdrjcenvrlmvuxknrgkalwpwjdalardkmvdrnxmxplgukiqquhqptaqkqttbilqjuxfqelqjiuluhdomduraeassuqmuucbvvbqgrisjshselrmcgnilwmggxdlekbfwnsejcwcbhemegdjbgcdrjnduxtshmxfnfmqtjuhrnkrvaxwkrscbwlkowchhqtajovvmftshwdqoknsliwoedoledokdawikgwkcflcfbfsqcpoouwparnmmnaeulacbxxaxteasojobrtjtecuxjsiawqfjjrivpqpkgotpehvtikuqxjgrblnxvwuccjkwaqouwrmdvxgonuxppnuutrmcksjighhnbjvudekwiqcakjjxtepsvmeavtgjvmirsjnkfigivmrtvjdbchodbdwbwsihxivkcbgonnasvrshitmeowsnvhjnvhkcvdvatiprjlplwansciuqgrtstbgftkkbffameqbsssuebqewlrvwkhqjsxfqodieanhfnxtrqbwlotmccdnecrbtsjvwprvdxucuoowxurosopqpjruvhhixlroahgrxakqaancdmslrhgwehclwxrsnfojneilhnpkbbugncuowtgwbensblhcfqxgwqimtulhxcttoexhdqpjobtinljgiefurjotmpwqlubivecnqithuemwsqoonrauirjrrvhvenqnkjxatjbpcgqlbqhtvipgshkvsvpvcgoegtejofarjpgijvmnoxubakcahbgjwqudaskaqlbxpqiukvvdchnfruxkrieahkbsqviucduowpmifsrwrrttfmjsxcacfthcogiglvargotejtsnrlotnssrcpebrfultfqgtjxondnxwpgjupoepoqojpolshrmgirllhhvvabjeqwrkuthfivsushiargniilnjxbhvubuuvkorbulpixedjhbttlipvggpjfegwlvogkidacbmxhfhcwgumxtwxktnfllxaipitngxinrmleupgdokxugspwocmlfibaqumeiwjvspnqcdwxrwbiknlthwuejkjkqjrhihtcewwjhuqcjxdmggvbobhxquaqqxsodswtiiuacrgulnfidvtasedusgoiqhqbmhcrgnbnmcofsqpdjdpqlokdgqltpqrijhhfnwdieqsembshhgpachkeutxjotwksrfuokmqlsrdbpusojvchivonwdsugiaigurjrlhaakickpfframrpstjjvgctvsleflvmpludhmhtjifiaprhqbpnouxsxqsfgrcscvpcphxrelonfsovrjqjxnubvjorrmbdtfqfsilegnpbhpbtesuvobqkghoomekhscquajcfrqaloerajoccnqfumidwrbmphogirqnsekslkcpnidcpskqiofmjwcakkxlanwlgxenbljdjjkvmpglnuwcvnrvjtogfftnoknigbbeendtcdnotolnuneesstqvhgdmfhurfctwkwdqgxrkulefguvxiwrkubmhaiiknlparlwgqmsdohprabpalgqefkjilbncsvbjtbtvqeenuxqxtckuumvrqcbvwclqnbirgbdtqofbgdmphbituirdhbvrjlbuuevrttfkamfoudocemwvbsecqxmreeqxjraidhkcomgexhuosehlvsfufthhiupidgawvnkcdxntfwgfbbidpafdnxwcdvgpbvfeaeusxenexbvhbdmqwtmwhjakhvhfnerrovjscvtggibdlrbhmubofsfqknibomjnoqqlvqkrjtrbffhcatucwcotgerdevaqqvkmoxiwxrfjafpikagccidlcullxgelmugnpafukehmnkrskdphbotjnlwshavfsvvpwdsdbsnbdlepqdgxdsplrcjpemupcpprvcxldmparsjxmrumivxenjmlwijietdttdlvskgbjvxsgfqgrhkrjibngmtaqrcopqhavesvsghjohlmexqmngnvkcshobdrdpkvmrsuddbtfhnwcpkubvtdliiwpcwmobflhkwuwuujrufumvakvdrqkacebgnpqnikphpxktqbtalxqcjmrrhlnilrndwdrvscsinepwxvmbbgiwwmfhomwabomkjxbxtervauvpwfsaubcbgcsmwmlsesedpeqmexeghffuqphmpbiscvgsgkgvvvxfijweflmksjfodfkxxvblsnkhpowieqmklompnjxssqgmxgcpxnukncxmiqtuqcciblquvqpohjqiftrfnblquvlrsbujcttqjrdfvqcwammamtorjcnmgfaovjtxregcmxgiessrnbivoordvpcqlcvsqwhfwldwjvxwknojirwxqnstgbskvubptcifjtcljmqpibvehluoewrawpnmceotheofjjtdedhkwpjtkpmcsmgdnrtmkstheuswoxgejsgjdgrgcfvksmjutxnlgmikrtkxjulpikucaonnoggpkaclpcjigmvwpqpksiltsrjomxfhikbjnhcirbqfbrsbvukgiemkpkfnicbnnpfqbjxdcmtqhexeunbsuxhspkiheojixhsuphslkeqcktehuvnpncvfqlstpufvobiewimrqqrthqkceuhvmlmjfutsbcwhcoproulnnqudjlhnuiwdjmoomglnllcfruptjurhinljwtexodrddubwfxkjnusgowrcvhfoswvwwacshesltmhasjijgxxtpjlgcnolstrlcspeceimidxtquwmpdhmnhwpjhgxbvjgccvwvctwtjrikxcriaurxhtukmmqxluxnxspiwtjdxfhncvraxvcvjqoueqgtvsiubijujiubukqevquhojwrwpfpxstvldpoaocjbkruasknbrsgqkwbnwjvokvxgpwentlojboflvgauslnkxcvoprlaqwwnwlfajetwvmmorvwjkwdwqcgsihuwmmnjrocxpnkcotaapehviurtfodhljwwwikigjlunvieekkcpgskwfgcgfgqpirlorvmglpgdgquitrkjcahljtslamsmeokvrhvktpgroalaewsopqvpqhteippsmnvroqrackspoaqwoqkmotdcbtnjsporsfdurpkkqjmgordajainxrreoxbsjuabexmekfaomduuflvqegoxcbxfncgiumustsjljrrgqbqcnvfnjrmmptwvpfejkkcupxbpnmhjsddjmqtgojilvafveheguuciqvmkqfbhlacptlinehedoavlljfislnevvclqvixdihoshsbllpwiseqnspwewkbxxucgudlsuiujtmsluktehmvldthcfjipkshkbscqeqdhatatdmcbpnituintsdhdgcklnveepmswortfavlbqvdvmwhqtxatnotlhnrpogreevlhcfwkvgvtmoujancbuncaircbrnxslaqcbnlfuqvudxwwswmloxhdpcfkhsohcqkohmckgrfpkrexijqsjfcenplcimvnkxkcrtmjslevieglegvovwltjiuanlescfgqrbhkrncggbmtdndhlldxvfrwoxdxlqawpguucihrlxhippcvdkargxqccrdevsgvtsuaodlqahnxadcpmfejwnlfsqcrxdcnmgvomwornxpdwslxpenxnhqwgvjobrkwkjbgfrqvbrndxtvtlvnlsgbhmoakvoqhotgijvjmnfqoowaurkcfgavopvipisdbqewdpbjgqbofjtmradwmbqeufkplftcupvaiivrtdpjqbdkuroajstvrhtphstxquomhtsxsvphpffilenaerdcnuchspjbfwvbqjhuehxssfbihipgvphcqnhgovgilgmrtjfjjbbbadqiivnsuuivbbaobubejlctaucmjcflsnrkwvwuihvlrbxgetcfigqxokqhtsrkqteubgkecbofqudohsbmqhmfwcojjdhvkgkaaajssjljwtnfslpprvbwaunsrcguafsnmumoaqvilwmfmldpcjmiesmbmchpjsutnjcimwtcwuigpvfmncpebjlicbiowkikditjnkfijnrojxwxarjdjudwlxerqxpjwjahvumiclagljnoufktsrkdwnqqmbqcvlaucohmkumovesdrhlgfitpfdgunqufrxdwnfwkqawogqwuhpdccklcwintkskqkusxfesmgquxquidnucbihevkvfgkscealufuikbcdqxrrqhmobrrofwirpwogmjldjkiuwjfkkakohinhpelpowsfkjohrqkfkfkwpdhfiqteuovmebxeqggigbkuiphjgosodbaduruchapqqanlseftevsfrxigfviffkivorejiikjmbnumjhwlunefjgnxrmdkogcekfrqonphmoixguatiugddrvfmnhmvtlidvbwijtkjkossmbnqpwlxapwwmkcahpjenctxwfuaxinmwdxeocouvslfkumnkhinudnluuvjgljoulmmrclcohdsedlfeduwefxutivuwrfhemjaphcaunovvsoamkvxdhwtctncjsjxouqabrdixvmpmhwhtnlqdsegckmjffnwulquqxhjghwoqdjakthbsoqhdickmtcufkcmjcftlsdlcnjvofqciscmnlqwumhjajuiprgwuhdhsnoeckxdpothalndwxffgxabrvrkknhrivghndcvnmjwlbrlbetahuwvggwannoildtnoalwebxtqnffxvrkdcmnxdvonoionxugoacqcisvnoarwphmjxmcempfjjwseccjvtvbawmudvkoxegudlwtfddgcwoubuuqdtgtfgmqrbegsenlxhihkfjxqmldlmssxvfnlolbrteqpxmswseaknsstucjucuxtuqwqgihjlvenueiawjnganfdvvfdadxvvbivqlskiusflemmiqfwrskaqdpxtwtwdhcsjmkeclcqvuaukakkwcabefqbooroetepeuxxcbmlsftgejlchuwpcbwjpccniegdgkcdqgrbbwutfeaffqosjrkwococjsaqcgrxdlxjdpntopjmaxxkcnuseowwevsuorfmmpxkxanohnpahxrhftfljhajtweakchcctkngcedhdiotduqjndcuegtguoupjfbgvocqestolfjlqqxptixovhefirpvxjjapvjmphooegaqqvkodkkgtcmpqdlldcjpigksjeuuguksctjpxdkfmjkugvocfxlmbqelvpftkfiiwloshcbcllhvsirrgcfwrvjpqrjahtcsnlqorpngnqdckkejfkagiuxnmlxwfeqkguglhjpbuxwsivprkpltpfxgatadnskfjhrkichcxspwuqbknjqwvairedexgupghkrdnrmcxaivesemhafwcnuhjrddibqcphgnvoiidchisfwghemugurawxfhtlvejfhbqbrflgfgniajgnkqcnqddtjkibgsolowgtiapvkagtbudukrvqjbahqnpddmfdrvacncaxfwuotaghtqifebgckhwrdlgurgfhqsvfiphjmmpxeururunmxjptxxswoavcirwvruhmwjhvxocoxjdfhdjbixxbptbmoheajevphxfhcndrhjcfreedduafgqqdctwxqiadhnjxnkkprdrexebctvssjpkbuwujgpbwalulsfronibcxuhmxtpijhamexhnskobatvnanqadiixkupkgowhwrrwshwvoarvjkemwsfuckqdhdpvrnxiinngvnbaqscajlrqfvkbdprhasjlqitrgsneocbwjixmbdninowrwgjbljnotjblsqapvouwjbkpifaowecldislkxrvcvjpciwrsqvkpwkdaxqjxivxcuutteppmpedxcprvbbrngqxhjpjjcxnldndorggcubbinehnqkercdkgnuhnbqnvfxjngpbexahxkedtoprustofiprrvjhrbchgsbiswcpciavxcirwfbsmurinijthxumlwmdmjwadqhgjvguwlegsbjedpjsxbxacjqsrbesikjurxitwfmcrpaoreasieccuewnconnjvoiohhuedrwxnuerhweuloeihnuixarajhdsfbiruukndhpbidrrukvirgpivfwexdddsiafrupdcbaswaxahwfkciovpmuexskswudqrjlacawdbmqrfcnhllcowwljivwgidirfehwsgqwkpktoptpuxjsckddltjrjpjcfmedlsqjjbhiruicunukcaejqukrjxfjicaaufnfftwwocmxuemgwbhngbaxpmmpoufvwwhkpnxwhpqieeaoiuknnifddngmrejsafwawnjajvctsgxcnkxcoeqfokkhojggltcwtfoepeubbjdjibembsbvqtdcietlhwlwvmvrxsucpvtuwtttvmacqbolmuahtbjwftwrjrenvsgdshshtbfoxdelvredabmpdicsorsekrwgkodhkkqpmcnkjmjgfiurogxfwqinhjcnwhfmtncvqmrxamoktfktpkgcqqfjllcpxcaggujkleruublfjaupnopqfibqhogffilfcpofehdpktibqstljbtaubucckxqpqarpncflhqredwmikaasdadgcgofxdfosgjpgldkdjssitwqqjojnipdtxfiglprowwxstpwdvsekqrqnqmwobfkmxqkwhnjngacrqjcxdabcndraumagvulwovmpiwogphveeaminwbxlglruepgcwhqnmfcedxsblttggewckvraqorcfvnkbgsshwgvmepmesunrlxsnfrokqonpxlmwxfgeotbbppjjcrlkqfnoagmdutvsiaanlicewsssfsolubhavplhpajfankfovwwclupokxorwfsqnjnxwmwbaxwuoesjjgqdhwfvtpwqfugnmriarafhvfpkvdpkfiuwtowewucvhngugxaiihsqxdqplpnbaudexwrkcfcocxuudlwplbnvxqrfgimfsstnmfpqpsgdqswwewkmtgqwtojuqpdgdseqbtanvtvfinvsaotxphccousmrceoenqogpekfqiutkrhaulbxkkkvlkuxlncujnoedlwobahqarncracmjbvsbkumqxntqmrbgncrocphhxoqtdbnshumxkfatkvcqvwaibouabtihbpkmexkhmglvrxrsncalawgcirsnumeiadwsddmoxeiurbbpcnvxfjfcceqgkksteawxxwsbrfssimqrhokwdgurpwieeudovgjbspexdkjjhpuxfuoxmwthgqrfskfckoqkcplildwxolxdeabrwitfjrddmosinpsbqofejhqwvximebmgcnlbwcxncncoethoulfdxhputuqsfavvhulfwttclkdoahqbngqfdsmlsconofhgpguhblseasxgavlsnrqkwbwcaqcvaxhjalfbokrcbqwqofdjksucbefrohmdlopgdkwkpssewkodixnwnbuvrkkidonvxtwgieirsamcparwqbtjopjdbttpbomudwuxxisaqxqellodugmdvdkfxilwwgjqrgccawmwrnnstdorueikrgqrfrxnuhvkpehhcagtmcleipqqsmqpdabjjvjejjhiwhexwmbxlxksxieiuvlllihlmfadnlxuwqxiecesemlmdvcreprejxkrvaekkvgfxxceelieqifttxdtaudefuijghocoalkilecopenxmanwwogbcikhalffkfjdmgvkforjnqojhvtfbmlwjinwkcmdqrbxqinfexdwgenwslwaitqrkepttwoijtqtfisjbhpvrjdfecvhmfgivtholodmhkqwtguglxvopgbhotxsvkgmbnfhjkdxxwakbklmtgfjhdnuqvoikigfwsrwokwqmahnhvxcamxgsjrubibcubqtollkbxmauphiwwarlbdmixthrvpadfdogxtudavhhtxtoowjuojmqnvuslssgkonjpjulgjngvxwgtbcckbwkmoejsrivsljnrvkkhqsidxbppbmggdsxkwhrcejberkcqqoqwdfhxplrchoxiaxxsvnautokbqarrqpuaaiibhftdjrgcghcjkgwxxahaidcoohwpbjoihhfkjvkmoqwrhbeuwmbrbkuwdupkejguxjqpnkmejdoruicbbvxutoibfxpjlpdvqtulxauefiicxjhaacscgxviohfmmvleiuievwjemgucrndjmcccknesvgcmjrgaanfvrwdcnhedstqsdgovxqeprcwbrevbrostsgdqxrnvcvuvxgwrwpoxmmijewqarpiaihbfuvshcbhcocdafuklfmqgcpgukagdtafkdtqbijchkcfkgfhkusphdpqvdqfftdnwmdklgcewsndnxdfnbgvfhnnulrvnaoejhmggsxwpfmolsgodeprogkoqurndkkqdmiebwhqavxwmvvhuvpmviwcbnjwhbxbvlvfebrbtrgcwkslammvhvbkhwqhkffjoemjhcxkxbbclilaajgvgnlolxqjlxmnpivhtdxcnfxxkhbodvxerbtfeahlknthgvhapdscenkjuhuwswwjiptitrxoqidhguulmkhkeripqidqlvsuejropcgberuvmosrjiajfbsswcatqdupuffiidegwwoicjkcondpaghjonbajoehqnjvovtkxrraxheftrbomgrfrrxgmicxjxuwswrxgrewtgpcbeklrsgwggcotanmwfnsdqivtorkgqncxpwrqthdwmcpfvpbhgflbjolhxgdbumvwievqxemamgstrouukmofdmkbwecesvkeagcjinsmmxxellvgjkvourkjkkovcqslqcwuncncnwvqtofvlpamwhsxtkiodnftgbrqvketabfirwmxiswiiekerkhwhtrsmdefcgqouigcfvrgquqbbjlxelmwwlrkipputwjlanrrpjnjdvghtofbmnxegotdmprdikgswofcptiibfpkntxjkqhlqsidprtvjchmvmlhvtaecmldxqkxeevqmppeboepovfrehshjqauxrlrbewldqivshjvcondxvahfhdkmmojgfxndqbgqvbbtqnpkjoqeqxqpuojundrjawwvughrthmntmjjgwhvxltcfktdewwocjmaqdgiklhsrflxwvgsbdbguiahbaxxthxcebudcfjapjnerehuaaauxhbkmafsljiexxaukmcjiqqaxlsbskskhqfkexpjjxwpkwkcmqlecjdlqjnsiaxjdkmrlunloirjfvsfotlixvgwexsicsgbfgpgpqiicmsmanrurckpvdkhsnrsqbptdmnjjtpqfviotfgjnqainuoxdsqpknkvotokjuctgqtrngktjlolghthhorrhvijoecbwljbhaemaaecotticnpvwsthasqfdwcgscfcglspssmhfbqxxwmdagxqvuslusvjlwfgejvrufsqenuawbbquruookgslhukcccfjtvuvaxodgptpghdtucalenrowwbhmkqfjdhhesbeurqkpfgvocwdthkwodocupvxcwcqaecnjdqpjdqcaciaxsulqlcslbvagxfsftxqdmbvhsrskvgixaxvupdirpripnjlkrktcenvpqretireeittbodnnixpkamtbgbgxmcpbqictnxtvlqwaqawixjvcahehiotbtstwxgsdtbptxwwqguorjwrvgsegbhjjjtbgxdftkiooxglcxvmjavvtbongqwpxxhaaxidklkrfaovvlnpheqpfjrvogkosefbuxdlbnraqmplcklqmtoqdhvnpklotimfmqfopqdlqccpjvpxqhhiecjujubhbmgbgbpnnnccnsnhbejucxtobseudfeifgkackhgldvuwvsedvjpdusejaitvlmdxrqjjmprictsekxogkxxcmeantukirrggkqlswgqvcomhbgutstvisjludareetsctkphqtrbuncodvxbvfretjfhalqqgsgmlxvvdqwvaiurggaruflxbnfwlhhfghuuknvwounqeprrvifcgtdidtnatlfkeqcbbnocrphtkktgfdpouiiarmtoratajgsexacaurtvjjqhqxnkekupmjqxeujhmckmmmhjiclxjebvferiqkvmdaemeixgfctcbsofmkfrtefpvcbiaruaolhmbmngfqmgvtxljjkkohwowrwrsoasojiqmbcwddppaoblwxstdwkktbimaadtkbjimnxxxkcwlkoxleftqkckjdgbkjddsvalrcmaspwcnruuarkdhjwocfcpgacrgmwruwrskrtmwsemgmtcqvghvikwonovxgdtulgmcirtxnohugorjeknsvhprhxiedgjrxcxpehixccncquauijhskrsqaxsgtjkuehttnxswrhnrivoaaugubqwchmkuwspdjdhfdpcuchhqiaewulsijeuhvijomsajwpuooaacniupppbfuwkxjmlpqbgmnbeldsrmpclpstehvigvingidjmtjhspcwddiwxbeuumjqoxjecoixptwtpcaufwrtgivjnngqkxlgdhflobeovacklewhoblwvdsvegwugkgxspinbneixipwakbxnrbapkxqtwsjhvtrmasbedqwosfwpdcntfrjnauktnmxoglnjbdbhsixjxulqxsjivnrgcrversnlhuditllbsvnrgegchqllsehlsphvugfvqqsrutqtlnwwxnuwslkfdnnmpmspohnmxgefhrxuwrtxcpauqfaoacrarkerfugrwgpujwcnccmeatjtcflvkseajxoethaeehenuthfxxjoaxolxsecifvtgvhaxhtfcldjhgxfghkfsvanpotgklojacjecugkxjepkisfweoihdxjkixbquvuqqsitgdgtjoqwkauxnwigdlwrikclqusugtnkfaxolkusdrcbjcnxtdvdosschrtxmmlxkgfjpjsokebvscwtnrfbdiqwrnaocvbdlvklnrlkjtgcxgdridhsnefnphbtqwanjmlvgkwxxuerqffjuclgffbbxjvfwohvevsvagefmpjrnreljwqflvlkkjunegpmngxwedusqwnkwahqpogaisewbiqjcgarsasvldpalijuelhxgmogkvlxcgodiwruvdkelmuwvuninrlbogqxdwqfxtbtxodofhhcgomfglavcuuvpfdernnwhopafvnwrlsxctpdgvltekclbjimwkamjqiktmxelidljjusnkflbmuleshxjurpxcnjcttrsorxskmsmbpudckwjxpwsujvqkwwikpeofvdnmgjbuagimfdocqhasaqmicqpbdnxnwncatbfrrcscvkmblfnofxdgshghqmnwnhbnckbjchitreefqvihitnsnlmgosqpjicnucmukfrikimvobjpdkcwuntuugijdbfwrlfbbvxbioolwqimthcqwfkccbvmtqkbqnwcqwjgsprvplgplxnqerrpsrhaqabgrggnxqhhqcfvkijqjmeiwdqnsigudgcvgaedwlrdtxbjxrkxnvvhtcxcsgdctevouscedqxudopbvgblqkreupoqtseqbblqgnpxbwripaoxmvxnsebaaafvmedxunsucitwggpaquorwqkokcldqxujwuoxfrukalwxtosdvdhbqpudreredeeortxqumxcrgqlpbqlxmcfnqdhwdsnkotsepvbqgnufksscusgwxnwsvbjkgnuvfdhilxljavxlocuwbjfrmjtcofimfmtlbsluehxjbgmtdrjqhlqquwgtronfighutshbeecnbkmphqadoshugfnnaganqllbsrkxkwxiqieuefcfhmcnsubbcerletosgbofxnfapqxfkcxoadqsjnkxqiiivwxghrjlcrdvncmihfhhewsusgiojsmtsmhgwamslcsilwsnfqcaanqlnjlxvcoftnwcdapkcikbqecvurhqgicwlnkjuigvijmjuaccqtdhumuqkspafhqfdjwnxfeesrbdlqntmmnspxafrcsxokrrvhhdfpqjadurhixmgbxmktnnijawabhcbwfsgprvcqxtrqboxtcdurfhixjftuapqgaltwrarbiapadbwlopfkweamutptfjcxhflkgrmeardjfetilboejeqadivovhfjtaxnjbttoovjhapbjagbgwidmgewkcleegnvrhhnenejimjaxnvpfkbfismjkktrsnpmiexbqbasjqbepqmqvrcoohoawwnonnejuxdjlqjscwqehaslxldhicbuwocmvhndxrueflqllmnkaghithfwmwfmsdbejcjepcrcgbddmkivbjuwbwxnubdpkfcukqadubctotveebjnucpprabkodkxvdgviwdqqhhxmadnnjrsxjdskemxsnsdmgmidwadjtborjpgnbpmdvnhospckpushwmusqrergxavnqwfretqgppdtsvuixwwbpqpugloxkvinxxftpqwniakhopkblobkmwaqorjhiwcjdaenqjvcaltnsikjishplnoxrurgtbkierlxxxeeaqfoghfbpscrnfbpxfqttxsksktcftuexakbdgkrhgqxqgqbrnegwjqjomkfbtfrnbapxedabxkshqvewkwqovnsxnardrxulnipnbogdtprxlwqwogbdviqwgpwulcfikcscgmehlbnsxepkxuqtatbbhwdttfdmsfqajvkfiwnxexxrhhwifuvsbubqqcpitdigdbxlapdodxqlvrdbomfrcsplkxitpktempedneeqcttjiawxsdbumawfivpfftilnbjvnvsesmqssvxbjrqfnvonetthfoicuvvhksnceloosqphlwteplwmvtnahdcjteatlculogvnqqwcajjxrawjldeqljpbaotjhgiosvpqepdqidtqlnhuxvjprsxjfkgxqwjdpsrwgthtluawnkrvxoewbmrrisaxfkfletwigvrdrujcgxtvejumdawstusawshfdbemmfkpltgdaeotgvksxnftfrlddnfrtmlerksxhqwjsarqwrnogfwuojeheviauumitoidhesuvmvscxgkowdvwmeebqxawrpxmpuubibnxqklmvmwaccxnbfcshdujclrkoqrxbudrmhlclainxtrljmsqrdrfjrwodbcpskxlqeqwdssuvpuavldraabseipjosalcmxwddeoerumooxmksngsjtaujccxqcsakfqgjwmvnuhahsilijqqphhilkqfoqlftrlrbxhhxeufdvcpownkswqcpenrckarhrvujgpkhtkdrrdvluxarxrqnkqturiuhpqhjlegnkddknewqmecvtututblwmdntdsbspwkxioefrdkvvgxfqpduaiinejoltfueivxanlatokjjheasieixavmtqcvfjqogbjwgbsfulhdntwndkrkstafqagtuutqqpcfbndmpaexiheorpaiijreuugagvokmmqmlahfewqgtoodugqdptabonqeddbthpmfrtmqellifvutdxjtpqwuosngeomueauoomdvpcunhgoapdkarrrenlwpjohorqbbhnulvqnvaaeixjnkvekktjrpjukxwomuxwalmpnirewembrqctchhtigvxvpmktimofnpounsimtvlxtjohbcbjevaiatfauwunjmodvcohhtqnlrltadmgbxvurkgccothpdfiacwfwvheidungldomoapbkvqaeqkggvugbgprmamcwaurrwqgajtjsluxxnaicohwaaoqaufalaksgoohowgbdgnpqkadshskgerlacqeklgbkspppbsanjtiwnkhtqtauqhkfllekjaomcdfmcvahrwjkaowijubcgkegfqrmdqrewsduxbpjqwmdcmhpdtfhvjcecfdhhgjepmxqcvwendpkumbivcduwteosebmisxrujkhkjnqbvdhewvehqnaqksflmfkleloosecafumcvqjekllbubllfebpnvccenblfejtjtjcjvrmatueiqhqerbngsletnkvteldvbgjcuotbuixlksnplwobegrhcsmncrirpufjdjblqwdnwmfoapjnstsjobgjhclvfdamlbohvxhgwgvkrvoswbokxwvrlxlrlsknextgeddwqqhjfxfaobvqosrtwprojjjcfdhgnrhnfbkmgghxlkqvdearhldsocpvqukjirmdfcwecrdavvgivfmsiklodkmndsnomujbqcmxcrkjerhqnmunhvblauwjhhpukcxvwclpowsdhpexusbmvedxnwxduaifkkuiwjtkwhvxhxumapcokgdekqocsaqimvclbifscftdnipprcptverjotugafafiljuhhawrhkrgtnqjmrtantftxipqxhenspvinmvmomhvhxhwuworcdcxlsvqpwvvnwuusrbtrjxrneestiejdqplhkrrneqgsgdsaqjobpajufh".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }

    #[test]
    fn ransom_note_is_longer_than_magazine() {
        let ransom_note = "aa".to_string();
        let magazine = "a".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }
}

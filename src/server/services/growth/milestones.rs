use std::collections::HashMap;
use std::sync::RwLock;

// Milestone Tracker Implementation

#[derive(Clone, Debug, PartialEq)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub threshold: i32,
    pub metric_type: String,
    pub is_achieved: bool,
    pub achieved_at: Option<i64>,
}

pub struct MilestoneTracker {
    orders: RwLock<i32>,
    visitors: RwLock<i32>,
    milestones: RwLock<Vec<Milestone>>,
}

impl MilestoneTracker {
    pub fn new() -> Self {
        let mut initial_milestones = Vec::new();
        initial_milestones.push(Milestone {
            id: "m_order_1".to_string(),
            title: "First Sale!".to_string(),
            description: "You completed your first order!".to_string(),
            threshold: 1,
            metric_type: "orders".to_string(),
            is_achieved: false,
            achieved_at: None,
        });
        initial_milestones.push(Milestone {
            id: "m_order_10".to_string(),
            title: "10th Order!".to_string(),
            description: "Double digits! Keep it up.".to_string(),
            threshold: 10,
            metric_type: "orders".to_string(),
            is_achieved: false,
            achieved_at: None,
        });
        initial_milestones.push(Milestone {
            id: "m_visitor_100".to_string(),
            title: "100 Visitors Today!".to_string(),
            description: "Your storefront is getting noticed.".to_string(),
            threshold: 100,
            metric_type: "visitors".to_string(),
            is_achieved: false,
            achieved_at: None,
        });

        MilestoneTracker {
            orders: RwLock::new(0),
            visitors: RwLock::new(0),
            milestones: RwLock::new(initial_milestones),
        }
    }

    pub fn record_order(&self) -> Vec<Milestone> {
        let mut ord = self.orders.write().unwrap();
        *ord += 1;
        let current_val = *ord;

        self.check_milestones("orders", current_val)
    }

    pub fn record_visitors(&self, count: i32) -> Vec<Milestone> {
        let mut vis = self.visitors.write().unwrap();
        *vis += count;
        let current_val = *vis;

        self.check_milestones("visitors", current_val)
    }

    fn check_milestones(&self, metric_type: &str, current_value: i32) -> Vec<Milestone> {
        let mut unlocked = Vec::new();
        let mut milestones = self.milestones.write().unwrap();

        for m in milestones.iter_mut() {
            if m.metric_type == metric_type && !m.is_achieved && current_value >= m.threshold {
                m.is_achieved = true;
                m.achieved_at = Some(1600000000);
                unlocked.push(m.clone());
            }
        }

        unlocked
    }

    pub fn get_all_milestones(&self) -> Vec<Milestone> {
        self.milestones.read().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milestone_progression() {
        let tracker = MilestoneTracker::new();

        let unlocked = tracker.record_order();
        assert_eq!(unlocked.len(), 1);
        assert_eq!(unlocked[0].title, "First Sale!");

        for _ in 2..10 {
            let u = tracker.record_order();
            assert_eq!(u.len(), 0);
        }

        let unlocked10 = tracker.record_order();
        assert_eq!(unlocked10.len(), 1);
        assert_eq!(unlocked10[0].title, "10th Order!");

        let vis_unlocked = tracker.record_visitors(99);
        assert_eq!(vis_unlocked.len(), 0);

        let vis_unlocked2 = tracker.record_visitors(2); // total 101
        assert_eq!(vis_unlocked2.len(), 1);
        assert_eq!(vis_unlocked2[0].title, "100 Visitors Today!");
    }
}


pub fn get_gamification_knowledge_0() -> String { String::from("Gamification is the process of integrating game design elements and principles into non game contexts The goal is to increase") }
pub fn get_gamification_knowledge_1() -> String { String::from("user engagement and motivation through the use of game elements such as points badges leaderboards and more It is a") }
pub fn get_gamification_knowledge_2() -> String { String::from("component of system design Gamification has been used to improve organizational productivity flow learning crowdsourcing knowledge retention employee recruitment and") }
pub fn get_gamification_knowledge_3() -> String { String::from("evaluation usability usefulness of systems physical exercise tailored interactions and icebreaker activities in dating apps traffic violations voter apathy public") }
pub fn get_gamification_knowledge_4() -> String { String::from("attitudes about alternative energy and more Techniques Gamification techniques work by leveraging people s desires for socializing learning mastery competition") }
pub fn get_gamification_knowledge_5() -> String { String::from("achievement status self expression altruism and closure or simply their response to framing a situation as a game Players are") }
pub fn get_gamification_knowledge_6() -> String { String::from("engaged using competition and rewards for completing tasks Rewards can include points badges levels filling a progress bar and virtual") }
pub fn get_gamification_knowledge_7() -> String { String::from("currency Making the rewards for accomplishing tasks visible to other players e g leaderboards encourages players to compete Meaningful choice") }
pub fn get_gamification_knowledge_8() -> String { String::from("onboarding tutorials increasing challenge and adding narrative are other ways to make tasks feel more like games Game elements Game") }
pub fn get_gamification_knowledge_9() -> String { String::from("elements are the building blocks of gamification applications They commonly include points badges leaderboards performance graphs meaningful stories avatars and") }
pub fn get_gamification_knowledge_10() -> String { String::from("teammates Points Points are basic elements of a multitude of games and gamified applications They are typically rewarded for the") }
pub fn get_gamification_knowledge_11() -> String { String::from("successful accomplishment of specified activities within the gamified environment and they serve to numerically represent a player s progress Various") }
pub fn get_gamification_knowledge_12() -> String { String::from("kinds of points can be differentiated between e g experience points redeemable points or reputation points as can the different") }
pub fn get_gamification_knowledge_13() -> String { String::from("purposes that points serve One of the most important purposes of points is to provide feedback Points allow the players") }
pub fn get_gamification_knowledge_14() -> String { String::from("in game behavior to be measured and they serve as continuous and immediate feedback and as a reward Badges Badges") }
pub fn get_gamification_knowledge_15() -> String { String::from("are defined as visual representations of achievements and can be earned and collected within the gamification environment They confirm the") }
pub fn get_gamification_knowledge_16() -> String { String::from("players achievements symbolize their merits and visibly show their accomplishment of levels or goals Earning a badge can be dependent") }
pub fn get_gamification_knowledge_17() -> String { String::from("on a specific number of points or on particular activities within the game Badges have many functions serving as goals") }
pub fn get_gamification_knowledge_18() -> String { String::from("if the prerequisites for winning them are known to the player or as virtual status symbols In the same way") }
pub fn get_gamification_knowledge_19() -> String { String::from("as points badges also provide feedback in that they indicate how the players have performed Badges can influence players behavior") }
pub fn get_gamification_knowledge_20() -> String { String::from("leading them to select certain routes and challenges in order to earn badges that are associated with them Additionally as") }
pub fn get_gamification_knowledge_21() -> String { String::from("badges symbolize one s membership in a group of those who own this particular badge they also can exert social") }
pub fn get_gamification_knowledge_22() -> String { String::from("influences on players and co players Leaderboards Leaderboards rank players according to their relative success measuring them against a certain") }
pub fn get_gamification_knowledge_23() -> String { String::from("success criterion As such leaderboards can help determine who performs best in a certain activity and are thus competitive indicators") }
pub fn get_gamification_knowledge_24() -> String { String::from("of progress that relate the player s own performance to the performance of others However the motivational potential of leaderboards") }
pub fn get_gamification_knowledge_25() -> String { String::from("is mixed Werbach and Hunter regard them as effective motivators if there are only a few points left to the") }
pub fn get_gamification_knowledge_26() -> String { String::from("next level or position but as demotivators if players find themselves at the bottom end of the leaderboard Competition caused") }
pub fn get_gamification_knowledge_27() -> String { String::from("by leaderboards can create social pressure to increase the player s level of engagement and can consequently have a constructive") }
pub fn get_gamification_knowledge_28() -> String { String::from("effect on participation and learning However these positive effects of competition are more likely if the respective competitors are approximately") }
pub fn get_gamification_knowledge_29() -> String { String::from("at the same performance level Performance graphs Performance graphs provide information about the players performance compared to their preceding performance") }
pub fn get_gamification_knowledge_30() -> String { String::from("during a game Thus in contrast to leaderboards performance graphs do not compare the player s performance to other players") }
pub fn get_gamification_knowledge_31() -> String { String::from("but instead evaluate the player s own performance over time Unlike the social reference standard of leaderboards performance graphs are") }
pub fn get_gamification_knowledge_32() -> String { String::from("based on an individual reference standard By graphically displaying the player s performance over a fixed period they focus on") }
pub fn get_gamification_knowledge_33() -> String { String::from("improvements Motivation theory postulates that this fosters mastery orientation which is particularly beneficial to learning Meaningful stories Meaningful stories are") }
pub fn get_gamification_knowledge_34() -> String { String::from("game design elements that don t relate to the player s performance The narrative context in which a gamified application") }
pub fn get_gamification_knowledge_35() -> String { String::from("can be embedded contextualizes activities and characters in the game and gives them meaning beyond the mere quest for points") }
pub fn get_gamification_knowledge_36() -> String { String::from("and achievements A story can be communicated by a game s title e g Space Invaders or by complex storylines") }
pub fn get_gamification_knowledge_37() -> String { String::from("typical of contemporary role playing video games e g The Elder Scrolls Series Narrative contexts can be oriented towards real") }
pub fn get_gamification_knowledge_38() -> String { String::from("non game contexts or act as analogies of real world settings The latter can enrich boring barely stimulating contexts and") }
pub fn get_gamification_knowledge_39() -> String { String::from("consequently inspire and motivate players particularly if the story is in line with their personal interests Avatars Avatars are visual") }
pub fn get_gamification_knowledge_40() -> String { String::from("representations of players within the game or gamification environment Usually they are chosen or even created by the player Avatars") }
pub fn get_gamification_knowledge_41() -> String { String::from("can be designed quite simply as a mere pictogram or they can be complexly animated three dimensional representations Their main") }
pub fn get_gamification_knowledge_42() -> String { String::from("formal requirement is that they unmistakably identify the players and set them apart from other human or computer controlled avatars") }
pub fn get_gamification_knowledge_43() -> String { String::from("Avatars allow the players to adopt or create another identity and in cooperative games to become part of a community") }
pub fn get_gamification_knowledge_44() -> String { String::from("Teammates Teammates whether they are other real players or virtual non player characters can induce conflict competition or cooperation The") }
pub fn get_gamification_knowledge_45() -> String { String::from("latter can be fostered particularly by introducing teams i e by creating defined groups of players that work together towards") }
pub fn get_gamification_knowledge_46() -> String { String::from("a shared objective Meta analytic evidence supports that the combination of competition and collaboration in games is likely to be") }
pub fn get_gamification_knowledge_47() -> String { String::from("effective for learning Game element hierarchy The described game elements fit within a broader framework which involves three types of") }
pub fn get_gamification_knowledge_48() -> String { String::from("elements dynamics mechanics and components These elements constitute the hierarchy of game elements Dynamics are the highest in the hierarchy") }
pub fn get_gamification_knowledge_49() -> String { String::from("They are the big picture aspects of the gamified system that should be considered and managed however they never directly") }
pub fn get_gamification_knowledge_50() -> String { String::from("enter into the game Dynamics elements provide motivation through features such as narrative or social interaction Mechanics are the basic") }
pub fn get_gamification_knowledge_51() -> String { String::from("processes that drive the action forward and generate player engagement and involvement Examples are chance turns and rewards Components are") }
pub fn get_gamification_knowledge_52() -> String { String::from("the specific instantiations of mechanics and dynamics elements like points quests and virtual goods Applications Marketing In November 2011 Australian") }
pub fn get_gamification_knowledge_53() -> String { String::from("broadcast and online media partnership Yahoo 7 launched its Fango mobile app SAP which TV viewers use to interact with") }
pub fn get_gamification_knowledge_54() -> String { String::from("shows via techniques like check ins and badges Gamification has also been used in customer loyalty programs In 2010 Starbucks") }
pub fn get_gamification_knowledge_55() -> String { String::from("gave custom Foursquare badges to people who checked in at multiple locations and offered discounts to people who checked in") }
pub fn get_gamification_knowledge_56() -> String { String::from("most frequently at an individual store Gamification also has been used as a tool for customer engagement and for encouraging") }
pub fn get_gamification_knowledge_57() -> String { String::from("desirable website usage behaviour Additionally gamification is applicable to increasing engagement on sites built on social network services For example") }
pub fn get_gamification_knowledge_58() -> String { String::from("in August 2010 the website builder DevHub announced an increase in the number of users who completed their online tasks") }
pub fn get_gamification_knowledge_59() -> String { String::from("from 10 to 80 after adding gamification elements On the programming question and answer site Stack Overflow users receive points") }
pub fn get_gamification_knowledge_60() -> String { String::from("and or badges for performing a variety of actions including spreading links to questions and answers via Facebook and Twitter") }
pub fn get_gamification_knowledge_61() -> String { String::from("A large number of different badges are available and when a user s reputation points exceed various thresholds the user") }
pub fn get_gamification_knowledge_62() -> String { String::from("gains additional privileges eventually including moderator privileges Gamification can be used for ideation structured brainstorming to produce new ideas A") }
pub fn get_gamification_knowledge_63() -> String { String::from("study at MIT Sloan found that ideation games helped participants generate more and better ideas and compared it to gauging") }
pub fn get_gamification_knowledge_64() -> String { String::from("the influence of academic papers by the numbers of citations received in subsequent research Health Applications like Fitocracy and QUENTIQ") }
pub fn get_gamification_knowledge_65() -> String { String::from("Dacadoo use gamification to encourage their users to exercise more effectively and improve their overall health Users are awarded varying") }
pub fn get_gamification_knowledge_66() -> String { String::from("numbers of points for activities they perform in their workouts and gain levels based on points collected Users can also") }
pub fn get_gamification_knowledge_67() -> String { String::from("complete quests sets of related activities and gain achievement badges for fitness milestones Health Month adds aspects of social gaming") }
pub fn get_gamification_knowledge_68() -> String { String::from("by allowing successful users to restore points to users who have failed to meet certain goals Public health researchers have") }
pub fn get_gamification_knowledge_69() -> String { String::from("studied the use of gamification in self management of chronic diseases and common mental disorders STD prevention and infection prevention") }
pub fn get_gamification_knowledge_70() -> String { String::from("and control In a review of health apps in the 2014 Apple App Store more than 100 apps showed a") }
pub fn get_gamification_knowledge_71() -> String { String::from("positive correlation between gamification elements used and high user ratings MyFitnessPal was named as the app that used the most") }
pub fn get_gamification_knowledge_72() -> String { String::from("gamification elements Further many applications have been proposed to reduce the impact of low air quality on health Work Gamification") }
pub fn get_gamification_knowledge_73() -> String { String::from("has been used in healthcare financial services transportation government and others Game elements such as experience points XP badges and") }
pub fn get_gamification_knowledge_74() -> String { String::from("other progress indicators have been shown to enhance user engagement and productivity in business learning programs Gamification can enhance employee") }
pub fn get_gamification_knowledge_75() -> String { String::from("engagement motivation and skill development by incorporating elements such as challenges progress tracking and rewards However gamification can also build") }
pub fn get_gamification_knowledge_76() -> String { String::from("resentment and drive unsafe personal behavior in the workplace such as workers skipping bathroom breaks Crowdsourcing Crowdsourcing has been gamified") }
pub fn get_gamification_knowledge_77() -> String { String::from("in games like Foldit a game designed by the University of Washington in which players compete to manipulate proteins into") }
pub fn get_gamification_knowledge_78() -> String { String::from("more efficient structures A 2010 paper in science journal Nature credited Foldit s 57 000 players with providing useful results") }
pub fn get_gamification_knowledge_79() -> String { String::from("that matched or outperformed algorithmically computed solutions The ESP Game is a game that is used to generate image metadata") }
pub fn get_gamification_knowledge_80() -> String { String::from("Google Image Labeler is a version of the ESP Game that Google has licensed to generate its own image metadata") }
pub fn get_gamification_knowledge_81() -> String { String::from("Research from the University of Bonn used gamification to increase wiki contributions by 62 In the context of online crowdsourcing") }
pub fn get_gamification_knowledge_82() -> String { String::from("gamification is also employed to improve the psychological and behavioral consequences of the solvers According to numerous research adding gamification") }
pub fn get_gamification_knowledge_83() -> String { String::from("components to a crowdsourcing platform can be considered as a design that shifts participants focus from task completion to involvement") }
pub fn get_gamification_knowledge_84() -> String { String::from("motivated by intrinsic factors Since the success of crowdsourcing competitions depends on a large number of participating solvers the platforms") }
pub fn get_gamification_knowledge_85() -> String { String::from("for crowdsourcing provide motivating factors to increase participation by drawing on the concepts of the game Education and training Gamification") }
pub fn get_gamification_knowledge_86() -> String { String::from("in the context of education and training is of particular interest because it offers a variety of benefits associated with") }
pub fn get_gamification_knowledge_87() -> String { String::from("learning outcomes and retention Using video game inspired elements like leaderboards and badges has been shown to be effective in") }
pub fn get_gamification_knowledge_88() -> String { String::from("engaging large groups and providing objectives for students to achieve outside of traditional norms like grades or verbal feedback Online") }
pub fn get_gamification_knowledge_89() -> String { String::from("learning platforms such as Khan Academy and even physical schools like New York City Department of Education s Quest to") }
pub fn get_gamification_knowledge_90() -> String { String::from("Learn use gamification to motivate students to complete mission based units and master concepts There is also an increasing interest") }
pub fn get_gamification_knowledge_91() -> String { String::from("in the use of gamification in health sciences and education as an engaging information delivery tool and in order to") }
pub fn get_gamification_knowledge_92() -> String { String::from("add variety to revision A 2016 study found that gamification can help students learn more effectively especially when they are") }
pub fn get_gamification_knowledge_93() -> String { String::from("motivated by curiosity or enjoyment of the learning itself One study found that students who were more intrinsically motivated tended") }
pub fn get_gamification_knowledge_94() -> String { String::from("to benefit more from gamified learning while those focused mainly on external rewards didn t respond as strongly With increased") }
pub fn get_gamification_knowledge_95() -> String { String::from("access to one to one student devices and accelerated by pressure from the COVID 19 pandemic many teachers from primary") }
pub fn get_gamification_knowledge_96() -> String { String::from("to post secondary settings have introduced live online quiz show style games into their lessons Gamification has also been used") }
pub fn get_gamification_knowledge_97() -> String { String::from("to promote learning outside of schools In August 2009 Gbanga launched a game for the Zurich Zoo where participants learned") }
pub fn get_gamification_knowledge_98() -> String { String::from("about endangered species by collecting animals in mixed reality Companies seeking to train their customers to use their product effectively") }
pub fn get_gamification_knowledge_99() -> String { String::from("can showcase features of their products with interactive games like Microsoft s Ribbon Hero 2 A wide range of employers") }
pub fn get_gamification_knowledge_100() -> String { String::from("including the United States Armed Forces Unilever and SAP currently use gamified training modules to educate their employees and motivate") }
pub fn get_gamification_knowledge_101() -> String { String::from("them to apply what they learned in trainings to their job According to a study conducted by Badgeville 78 of") }
pub fn get_gamification_knowledge_102() -> String { String::from("workers are utilizing games based motivation at work and nearly 91 say these systems improve their work experience by increasing") }
pub fn get_gamification_knowledge_103() -> String { String::from("engagement awareness and productivity In the form of occupational safety training technology can provide realistic and effective simulations of real") }
pub fn get_gamification_knowledge_104() -> String { String::from("life experiences making safety training less passive and more engaging more flexible in terms of time management and a cost") }
pub fn get_gamification_knowledge_105() -> String { String::from("effective alternative to practice Additionally the combined use of virtual reality and gamification can provide more effective solutions in terms") }
pub fn get_gamification_knowledge_106() -> String { String::from("of knowledge acquisition and retention when they are compared with traditional training methods Technology design Traditionally researchers thought of motivations") }
pub fn get_gamification_knowledge_107() -> String { String::from("to use computer systems to be primarily driven by extrinsic purposes however many modern systems have their use driven primarily") }
pub fn get_gamification_knowledge_108() -> String { String::from("by intrinsic motivations Examples of such systems used primarily to fulfill users intrinsic motivations include online gaming virtual worlds online") }
pub fn get_gamification_knowledge_109() -> String { String::from("shopping learning education online dating digital music repositories social networking online pornography and so on Such systems are excellent candidates") }
pub fn get_gamification_knowledge_110() -> String { String::from("for further gamification in their design Moreover even traditional management information systems e g ERP CRM are being gamified such") }
pub fn get_gamification_knowledge_111() -> String { String::from("that both extrinsic and intrinsic motivations must increasingly be considered As illustration Microsoft has announced plans to use gamification techniques") }
pub fn get_gamification_knowledge_112() -> String { String::from("for its Windows Phone 7 operating system design While businesses face the challenges of creating motivating gameplay strategies what makes") }
pub fn get_gamification_knowledge_113() -> String { String::from("for effective gamification is a key question One important type of technological design in gamification is the player centered design") }
pub fn get_gamification_knowledge_114() -> String { String::from("Based on the design methodology user centered design its main goal is to promote greater connectivity and positive behavior change") }
pub fn get_gamification_knowledge_115() -> String { String::from("between technological consumers It has five steps that help computer users connect with other people online to help them accomplish") }
pub fn get_gamification_knowledge_116() -> String { String::from("goals and other tasks they need to complete The 5 steps are an individual or company has to know their") }
pub fn get_gamification_knowledge_117() -> String { String::from("player their target audience identify their mission their goal understand human motivation the personality desires and triggers of the target") }
pub fn get_gamification_knowledge_118() -> String { String::from("audience apply mechanics points badges leaderboards etc and to manage monitor and measure the way they are using their mechanics") }
pub fn get_gamification_knowledge_119() -> String { String::from("to ensure it is helping them achieve the desired outcome of their goal and that their goal is specific and") }
pub fn get_gamification_knowledge_120() -> String { String::from("realistic Authentication Gamification has also been applied to authentication Games have been proposed as a way for users to learn") }
pub fn get_gamification_knowledge_121() -> String { String::from("new and more complicated passwords Gamification has also been proposed as a way to select and manage archives Online gambling") }
pub fn get_gamification_knowledge_122() -> String { String::from("The merging of gambling and gamification referred to as gamblification has been used to some extent by online casinos Some") }
pub fn get_gamification_knowledge_123() -> String { String::from("brands use an incremental reward system to extend the typical player lifecycle and to encourage repeat visits and cash deposits") }
pub fn get_gamification_knowledge_124() -> String { String::from("at the casino in return for rewards such as free spins and cash match bonuses on subsequent deposits History The") }
pub fn get_gamification_knowledge_125() -> String { String::from("term gamification first appeared online in the context of computer software in 2008 Gamification did not gain popularity until 2010") }
pub fn get_gamification_knowledge_126() -> String { String::from("Even prior to the term coming into use other fields borrowing elements from videogames was common for example some work") }
pub fn get_gamification_knowledge_127() -> String { String::from("in learning disabilities and scientific visualization adapted elements from videogames The term gamification first gained widespread usage in 2010 in") }
pub fn get_gamification_knowledge_128() -> String { String::from("a more specific sense referring to incorporation of social reward aspects of games into software The technique captured the attention") }
pub fn get_gamification_knowledge_129() -> String { String::from("of venture capitalists one of whom said he considered gamification the most promising area in gaming Another observed that half") }
pub fn get_gamification_knowledge_130() -> String { String::from("of all companies seeking funding for consumer software applications mentioned game design in their presentations Several researchers consider gamification closely") }
pub fn get_gamification_knowledge_131() -> String { String::from("related to earlier work on adapting game design elements and techniques to non game contexts Deterding et al survey research") }
pub fn get_gamification_knowledge_132() -> String { String::from("in human computer interaction that uses game derived elements for motivation and interface design and Nelson argues for a connection") }
pub fn get_gamification_knowledge_133() -> String { String::from("to both the Soviet concept of socialist competition and the American management trend of fun at work Fuchs points out") }
pub fn get_gamification_knowledge_134() -> String { String::from("that gamification might be driven by new forms of ludic interfaces Gamification conferences have also retroactively incorporated simulation e g") }
pub fn get_gamification_knowledge_135() -> String { String::from("Will Wright designer of the 1989 video game SimCity was the keynote speaker at the gamification conference Gsummit 2013 In") }
pub fn get_gamification_knowledge_136() -> String { String::from("October 2007 Bunchball was the first company to provide game mechanics as a service on Dunder Mifflin Infinity the community") }
pub fn get_gamification_knowledge_137() -> String { String::from("site for the NBC TV show The Office Badgeville which offered gamification services launched in late 2010 and raised 15") }
pub fn get_gamification_knowledge_138() -> String { String::from("million in venture capital funding in its first year of operation Gabe Zichermann coined funware as an alternative term for") }
pub fn get_gamification_knowledge_139() -> String { String::from("gamification Gamification as an educational and behavior modification tool reached the public sector by 2012 when the United States Department") }
pub fn get_gamification_knowledge_140() -> String { String::from("of Energy co funded multiple research trials including consumer behavior studies adapting the format of Programmed learning into mobile microlearning") }
pub fn get_gamification_knowledge_141() -> String { String::from("to experiment with the impacts of gamification in reducing energy use Gamification 2013 an event exploring the future of gamification") }
pub fn get_gamification_knowledge_142() -> String { String::from("was held at the University of Waterloo Stratford Campus in October 2013 Regulation and policy A March 2022 consultation paper") }
pub fn get_gamification_knowledge_143() -> String { String::from("by the Board of the International Organization of Securities Commissions IOSCO questions whether some gamification tactics should be banned Reception") }
pub fn get_gamification_knowledge_144() -> String { String::from("The majority of scientific studies on gamification find it has positive effects on individuals However individual and contextual differences exist") }
pub fn get_gamification_knowledge_145() -> String { String::from("Criticism University of Hamburg researcher Sebastian Deterding has characterized the initial popular strategies for gamification as not being fun and") }
pub fn get_gamification_knowledge_146() -> String { String::from("creating an artificial sense of achievement He also says that gamification can encourage unintended behaviours Poorly designed gamification in the") }
pub fn get_gamification_knowledge_147() -> String { String::from("workplace has been compared to Taylorism and is considered a form of micromanagement In a review of 132 of the") }
pub fn get_gamification_knowledge_148() -> String { String::from("top health and fitness apps in the Apple app store in 2014 using gamification as a method to modify behavior") }
pub fn get_gamification_knowledge_149() -> String { String::from("the authors concluded that Despite the inclusion of at least some components of gamification the mean scores of integration of") }
pub fn get_gamification_knowledge_150() -> String { String::from("gamification components were still below 50 percent This was also true for the inclusion of game elements and the use") }
pub fn get_gamification_knowledge_151() -> String { String::from("of health behavior theory constructs thus showing a lack of following any clear industry standard of effective gaming gamification or") }
pub fn get_gamification_knowledge_152() -> String { String::from("behavioral theory in health and fitness apps Concern was also expressed in a 2016 study analyzing outcome data from 1") }
pub fn get_gamification_knowledge_153() -> String { String::from("298 users who competed in gamified and incentivized exercise challenges while wearing wearable devices In that study the authors conjectured") }
pub fn get_gamification_knowledge_154() -> String { String::from("that data may be highly skewed by cohorts of already healthy users rather than the intended audiences of participants requiring") }
pub fn get_gamification_knowledge_155() -> String { String::from("behavioral intervention Game designers like Jon Radoff and Margaret Robertson have also criticized gamification as excluding elements like storytelling and") }
pub fn get_gamification_knowledge_156() -> String { String::from("experiences and using simple reward systems in place of true game mechanics Gamification practitioners have pointed out that while the") }
pub fn get_gamification_knowledge_157() -> String { String::from("initial popular designs were in fact mostly relying on simplistic reward approach even those led to significant improvements in short") }
pub fn get_gamification_knowledge_158() -> String { String::from("term engagement This was supported by the first comprehensive study in 2014 which concluded that an increase in gamification elements") }
pub fn get_gamification_knowledge_159() -> String { String::from("correlated with an increase in motivation score but not with capacity or opportunity trigger scores The same study called for") }
pub fn get_gamification_knowledge_160() -> String { String::from("standardization across the app industry on gamification principles to improve the effectiveness of health apps on the health outcomes of") }
pub fn get_gamification_knowledge_161() -> String { String::from("users MIT Professor Kevin Slavin has described business research into gamification as flawed and misleading for those unfamiliar with gaming") }
pub fn get_gamification_knowledge_162() -> String { String::from("Heather Chaplin writing in Slate describes gamification as an allegedly populist idea that actually benefits corporate interests over those of") }
pub fn get_gamification_knowledge_163() -> String { String::from("ordinary people Jane McGonigal has distanced her work from the label gamification listing rewards outside of gameplay as the central") }
pub fn get_gamification_knowledge_164() -> String { String::from("idea of gamification and distinguishing game applications where the gameplay itself is the reward under the term gameful design Gamification") }
pub fn get_gamification_knowledge_165() -> String { String::from("as a term has also been criticized Ian Bogost has referred to the term as a marketing fad and suggested") }
pub fn get_gamification_knowledge_166() -> String { String::from("exploitation ware as a more suitable name for the games used in marketing Other opinions on the terminology criticism have") }
pub fn get_gamification_knowledge_167() -> String { String::from("made the case why the term gamification makes sense In an article by the Los Angeles Times the gamification of") }
pub fn get_gamification_knowledge_168() -> String { String::from("worker engagement at Disneyland was described as an electronic whip Workers had reported feeling controlled and overworked by the system") }
pub fn get_gamification_knowledge_169() -> String { String::from("Ethical Concerns Extremism and Mass Violence Some extremist websites have used gamification elements such as points status levels and avatars") }
pub fn get_gamification_knowledge_170() -> String { String::from("to encourage participation in their online forums The website 8chan has also been criticized for gamifying violence In 2019 three") }
pub fn get_gamification_knowledge_171() -> String { String::from("mass shooters starting with the Christchurch attacker posted to 8chan before launching their attacks The Christchurch shooter livestreamed his attack") }
pub fn get_gamification_knowledge_172() -> String { String::from("using a helmet cam and soundtrack making his livestream reminiscent of a first person shooter video game The Poway and") }
pub fn get_gamification_knowledge_173() -> String { String::from("El Paso shooters copied elements of this attack including posting to 8chan prior to their attacks The Poway shooter also") }
pub fn get_gamification_knowledge_174() -> String { String::from("attempted to livestream Since the Christchurch attack users on 8chan have discussed mass shootings in gamified terms referencing bodycounts as") }
pub fn get_gamification_knowledge_175() -> String { String::from("high scores to beat See also Bartle taxonomy of player types BrainHex Dark pattern Egoboo a component of some gamification") }
pub fn get_gamification_knowledge_176() -> String { String::from("strategies Gamification of learning GNS theory Notes References Further reading Burke Brian 2014 Gamify How Gamification Motivates People to Do") }
pub fn get_gamification_knowledge_177() -> String { String::from("Extraordinary Things Bibliomotion ISBN 978 1 937134 85 3 Marczewski Andrzej 2018 Even Ninja Monkeys Like to Play Unicorn Edition") }
pub fn get_gamification_knowledge_178() -> String { String::from("CreateSpace Independent Publishing ISBN 978 1 7240 1710 9") }
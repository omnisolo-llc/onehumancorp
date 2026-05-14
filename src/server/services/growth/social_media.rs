use std::collections::HashMap;
use std::sync::RwLock;

// Social Media Service Implementation

pub struct SocialMediaService {
    accounts: RwLock<HashMap<String, String>>,
    drafts: RwLock<HashMap<String, SocialPostDraft>>,
    scheduled_posts: RwLock<Vec<ScheduledPost>>,
}

#[derive(Clone)]
pub struct SocialPostDraft {
    pub id: String,
    pub content: String,
    pub platform: String,
    pub generated_by_ai: bool,
    pub approved: bool,
    pub created_at: i64,
}

#[derive(Clone)]
pub struct ScheduledPost {
    pub draft_id: String,
    pub scheduled_time: i64,
    pub platform: String,
}

impl SocialMediaService {
    pub fn new() -> Self {
        SocialMediaService {
            accounts: RwLock::new(HashMap::new()),
            drafts: RwLock::new(HashMap::new()),
            scheduled_posts: RwLock::new(Vec::new()),
        }
    }

    pub fn connect_account(&self, platform: &str, token: &str) -> Result<(), String> {
        if token.is_empty() {
            return Err("Token cannot be empty".to_string());
        }
        let mut acc = self.accounts.write().unwrap();
        acc.insert(platform.to_string(), token.to_string());
        Ok(())
    }

    pub fn is_connected(&self, platform: &str) -> bool {
        let acc = self.accounts.read().unwrap();
        acc.contains_key(platform)
    }

    pub fn generate_ai_draft(&self, platform: &str, topic: &str) -> Result<String, String> {
        if !self.is_connected(platform) {
            return Err(format!("Not connected to {}", platform));
        }

        let content = format!("Exciting news about {}! Check out our latest updates. #{} #growth", topic, topic.replace(" ", ""));
        let id = format!("draft_1"); // simplified

        let draft = SocialPostDraft {
            id: id.clone(),
            content,
            platform: platform.to_string(),
            generated_by_ai: true,
            approved: false,
            created_at: 0,
        };

        let mut drafts = self.drafts.write().unwrap();
        drafts.insert(id.clone(), draft);

        Ok(id)
    }

    pub fn approve_draft(&self, draft_id: &str) -> Result<(), String> {
        let mut drafts = self.drafts.write().unwrap();
        if let Some(mut draft) = drafts.get_mut(draft_id) {
            draft.approved = true;
            return Ok(());
        }
        Err("Draft not found".to_string())
    }

    pub fn edit_draft(&self, draft_id: &str, new_content: &str) -> Result<(), String> {
        let mut drafts = self.drafts.write().unwrap();
        if let Some(mut draft) = drafts.get_mut(draft_id) {
            draft.content = new_content.to_string();
            draft.approved = false;
            return Ok(());
        }
        Err("Draft not found".to_string())
    }

    pub fn schedule_post(&self, draft_id: &str, timestamp: i64) -> Result<(), String> {
        let drafts = self.drafts.read().unwrap();
        let draft = drafts.get(draft_id).ok_or_else(|| "Draft not found".to_string())?;

        if !draft.approved {
            return Err("Cannot schedule an unapproved draft".to_string());
        }

        let mut scheduled = self.scheduled_posts.write().unwrap();
        scheduled.push(ScheduledPost {
            draft_id: draft.id.clone(),
            scheduled_time: timestamp,
            platform: draft.platform.clone(),
        });

        Ok(())
    }

    pub fn publish_now(&self, draft_id: &str) -> Result<bool, String> {
        let drafts = self.drafts.read().unwrap();
        let draft = drafts.get(draft_id).ok_or_else(|| "Draft not found".to_string())?;

        if !draft.approved {
            return Err("Cannot publish an unapproved draft".to_string());
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_media_flow() {
        let service = SocialMediaService::new();
        assert!(service.generate_ai_draft("Instagram", "New Product").is_err());
        assert!(service.connect_account("Instagram", "ig_token_123").is_ok());
        assert!(service.is_connected("Instagram"));
        let draft_id = service.generate_ai_draft("Instagram", "New Product").unwrap();
        assert!(!draft_id.is_empty());
        assert!(service.publish_now(&draft_id).is_err());
        assert!(service.edit_draft(&draft_id, "Custom edited text!").is_ok());
        assert!(service.approve_draft(&draft_id).is_ok());
        assert!(service.schedule_post(&draft_id, 1600000000).is_ok());
        assert!(service.publish_now(&draft_id).unwrap());
    }

    #[test]
    fn test_edit_resets_approval() {
        let service = SocialMediaService::new();
        service.connect_account("Twitter", "tw_token").unwrap();
        let draft_id = service.generate_ai_draft("Twitter", "Update").unwrap();
        service.approve_draft(&draft_id).unwrap();
        service.edit_draft(&draft_id, "Wait, change this").unwrap();
        assert!(service.publish_now(&draft_id).is_err());
    }
}


pub fn get_social_media_knowledge_0() -> String { String::from("Social media marketing is the use of social media platforms and websites to promote a product or service Although the") }
pub fn get_social_media_knowledge_1() -> String { String::from("terms e marketing and digital marketing are still dominant in academia social media marketing is becoming more popular for practitioners") }
pub fn get_social_media_knowledge_2() -> String { String::from("and researchers Social media platforms such as Facebook LinkedIn Instagram and Twitter among others have built in data analytics tools") }
pub fn get_social_media_knowledge_3() -> String { String::from("that companies can use to track the progress success and engagement of social media marketing campaigns Companies address a range") }
pub fn get_social_media_knowledge_4() -> String { String::from("of stakeholders through social media marketing including current and potential customers current and potential employees journalists bloggers and the general") }
pub fn get_social_media_knowledge_5() -> String { String::from("public On a strategic level social media marketing includes the management of a marketing campaign governance setting the scope e") }
pub fn get_social_media_knowledge_6() -> String { String::from("g more active or passive use and the establishment of a firm s desired social media culture and tone Firms") }
pub fn get_social_media_knowledge_7() -> String { String::from("that use social media marketing can allow customers and Internet users to post user generated content e g online comments") }
pub fn get_social_media_knowledge_8() -> String { String::from("product reviews etc also known as earned media rather than use marketer prepared advertising copy Purposes and tactics Social media") }
pub fn get_social_media_knowledge_9() -> String { String::from("may be employed in marketing as a communications tool that makes companies accessible to those who are interested in their") }
pub fn get_social_media_knowledge_10() -> String { String::from("product and visible to those who are not familiar with their products It is used by companies to create buzz") }
pub fn get_social_media_knowledge_11() -> String { String::from("learn from customers and target them Of the top 10 factors that correlate with a strong Google organic search seven") }
pub fn get_social_media_knowledge_12() -> String { String::from("are social media dependent This means that if brands with little to no social media presence tend to show up") }
pub fn get_social_media_knowledge_13() -> String { String::from("less on Google searches While platforms such as Twitter Facebook and in the past Google have a larger number of") }
pub fn get_social_media_knowledge_14() -> String { String::from("monthly users the visual media sharing based mobile platforms garner a higher interaction rate in comparison and have registered the") }
pub fn get_social_media_knowledge_15() -> String { String::from("fastest growth and have changed the ways in which consumers engage with brand content Instagram has an interaction rate of") }
pub fn get_social_media_knowledge_16() -> String { String::from("1 46 with an average of 130 million users monthly as opposed to Twitter which has a 03 interaction rate") }
pub fn get_social_media_knowledge_17() -> String { String::from("with an average of 210 million monthly users Unlike traditional media that are often cost prohibitive to many companies a") }
pub fn get_social_media_knowledge_18() -> String { String::from("social media strategy does not require significant financial investment To this end companies make use of platforms such as Facebook") }
pub fn get_social_media_knowledge_19() -> String { String::from("Twitter YouTube TikTok and Instagram to reach audiences much wider than through traditional print television or radio advertisements alone at") }
pub fn get_social_media_knowledge_20() -> String { String::from("a fraction of the cost as most social networking sites can be used at little or no cost however some") }
pub fn get_social_media_knowledge_21() -> String { String::from("websites charge companies for premium services This has changed the ways that companies approach and interact with customers as a") }
pub fn get_social_media_knowledge_22() -> String { String::from("substantial percentage of consumer interactions are now being carried out over online platforms with much higher visibility Customers can post") }
pub fn get_social_media_knowledge_23() -> String { String::from("reviews of products and services rate customer service and ask questions or voice concerns directly to companies through social media") }
pub fn get_social_media_knowledge_24() -> String { String::from("platforms According to Measuring Success over 80 of consumers use the web to research products and services Thus social media") }
pub fn get_social_media_knowledge_25() -> String { String::from("marketing is also used by businesses in order to build relationships of trust with consumers To this aim companies may") }
pub fn get_social_media_knowledge_26() -> String { String::from("hire personnel to specifically handle these social media interactions who usually report under the title of online community managers Handling") }
pub fn get_social_media_knowledge_27() -> String { String::from("these interactions in a satisfactory manner can result in an increase of consumer trust To both this aim and to") }
pub fn get_social_media_knowledge_28() -> String { String::from("fix the public s perception of a company three steps are taken in order to address consumer concerns Identifying the") }
pub fn get_social_media_knowledge_29() -> String { String::from("extent of the social chatter Engaging the influencers to help Developing a proportional response Strategies Passive approach Social media can") }
pub fn get_social_media_knowledge_30() -> String { String::from("be a useful source of market information and a way to hear customers perspectives Blogs content communities and forums are") }
pub fn get_social_media_knowledge_31() -> String { String::from("platforms where individuals share their reviews and recommendations of brands products and services Businesses are able to tap into and") }
pub fn get_social_media_knowledge_32() -> String { String::from("analyze customer voices and feedback generated in social media for marketing purposes In this sense social media is a relatively") }
pub fn get_social_media_knowledge_33() -> String { String::from("inexpensive source of market intelligence which can be used by marketers and managers to track and respond to consumer identified") }
pub fn get_social_media_knowledge_34() -> String { String::from("problems and detect market opportunities Active approach Social media can be used as a public relations tool a direct marketing") }
pub fn get_social_media_knowledge_35() -> String { String::from("tool and a communication channel to target very specific audiences with social media influencers and social media personalities as effective") }
pub fn get_social_media_knowledge_36() -> String { String::from("customer engagement tools This tactic is widely known as influencer marketing which gives brands the opportunity to reach their target") }
pub fn get_social_media_knowledge_37() -> String { String::from("audience via a group of selected influencers advertising their product or service Brands were projected to spend up to 15") }
pub fn get_social_media_knowledge_38() -> String { String::from("billion on influencer marketing by 2022 per Business Insider Intelligence estimates based on Mediakix data The use of customer influencers") }
pub fn get_social_media_knowledge_39() -> String { String::from("such as popular bloggers can be an efficient and cost effective method to launch new products or services Engagement Engagement") }
pub fn get_social_media_knowledge_40() -> String { String::from("with the social web means that customers and stakeholders are active participants rather than passive spectators An example of these") }
pub fn get_social_media_knowledge_41() -> String { String::from("are consumer advocacy groups and groups that criticize companies e g lobby groups or advocacy organizations The use of Social") }
pub fn get_social_media_knowledge_42() -> String { String::from("media in a business or political context allows people to express and share opinions about a company s products services") }
pub fn get_social_media_knowledge_43() -> String { String::from("or business practices or a government s actions On social media each participant becomes part of the marketing department or") }
pub fn get_social_media_knowledge_44() -> String { String::from("a challenge to the marketing effort as other customers read their comments or reviews The effectiveness of social media marketing") }
pub fn get_social_media_knowledge_45() -> String { String::from("campaigns is dependent on the promotion of online engagement With the advent of social media marketing it has become increasingly") }
pub fn get_social_media_knowledge_46() -> String { String::from("important to gain customer interest in products and services which can eventually be translated into buying behavior or voting and") }
pub fn get_social_media_knowledge_47() -> String { String::from("donating behavior in a political context New online marketing concepts of engagement and loyalty have emerged which aim to build") }
pub fn get_social_media_knowledge_48() -> String { String::from("customer participation and brand reputation Engagement in social media for the purpose of a social media strategy is divided into") }
pub fn get_social_media_knowledge_49() -> String { String::from("two parts The first is proactive regular posting of new online content which can be seen through digital photos digital") }
pub fn get_social_media_knowledge_50() -> String { String::from("videos text and conversations It is also represented through sharing of content and information from others via weblinks The second") }
pub fn get_social_media_knowledge_51() -> String { String::from("part is reactive conversations with social media users responding to those who reach out to others social media profiles through") }
pub fn get_social_media_knowledge_52() -> String { String::from("comments or messages Campaigns Local businesses Small businesses use social networking sites as a promotional technique Businesses can follow individuals") }
pub fn get_social_media_knowledge_53() -> String { String::from("social media usage in their local area and advertise specials and deals which can be exclusive and in the form") }
pub fn get_social_media_knowledge_54() -> String { String::from("of get a free drink with a copy of this tweet This type of message encourages other locals to follow") }
pub fn get_social_media_knowledge_55() -> String { String::from("the business on their official websites in order to obtain the promotional deal The business s brand visibility is enhanced") }
pub fn get_social_media_knowledge_56() -> String { String::from("in the process Social networking sites are also used by small businesses to develop their own market research on new") }
pub fn get_social_media_knowledge_57() -> String { String::from("products and services By encouraging their customers to give feedback on new product ideas businesses can gain insights on whether") }
pub fn get_social_media_knowledge_58() -> String { String::from("or not a product may be accepted by their target market enough to merit full production In addition customers will") }
pub fn get_social_media_knowledge_59() -> String { String::from("feel the company has engaged them in the process of co creation the process in which the business uses customer") }
pub fn get_social_media_knowledge_60() -> String { String::from("feedback to create or modify a product or service to fill a need of the target market Such feedback can") }
pub fn get_social_media_knowledge_61() -> String { String::from("be presented in various forms such as surveys contests and polls Social networking sites such as LinkedIn also provide opportunities") }
pub fn get_social_media_knowledge_62() -> String { String::from("for small businesses to find candidates to fill staff positions Review sites such as Yelp help small businesses build their") }
pub fn get_social_media_knowledge_63() -> String { String::from("reputation beyond brand visibility Positive customer peer reviews help influence new prospects to purchase goods and services more than company") }
pub fn get_social_media_knowledge_64() -> String { String::from("advertising Benefits Social Media Marketing allows companies to promote themselves to large diverse audiences that could not be reached through") }
pub fn get_social_media_knowledge_65() -> String { String::from("traditional marketing such as phone and email based advertising Marketing on most social media platforms also comes at little to") }
pub fn get_social_media_knowledge_66() -> String { String::from("no cost making it accessible to virtually any size business Social Media Marketing accommodates personalized and direct marketing that targets") }
pub fn get_social_media_knowledge_67() -> String { String::from("specific demographics and markets Companies can engage with customers directly allowing them to obtain feedback and resolve issues almost immediately") }
pub fn get_social_media_knowledge_68() -> String { String::from("Another advantage of social media marketing is that it s an ideal environment for a company to conduct market research") }
pub fn get_social_media_knowledge_69() -> String { String::from("It can be used as a means of obtaining information about competitors and boost competitive advantage Social platforms can be") }
pub fn get_social_media_knowledge_70() -> String { String::from("used to promote brand events deals and news Social platforms can also be used to offer incentives in the form") }
pub fn get_social_media_knowledge_71() -> String { String::from("of loyalty points and discounts It allows companies to build an online platform to promote and sell their product Social") }
pub fn get_social_media_knowledge_72() -> String { String::from("media marketing can also be useful for gaining customers who wouldn t know about the business otherwise It builds a") }
pub fn get_social_media_knowledge_73() -> String { String::from("community that enhances a business s reach in their selected target market Advertising campaigns To promote the 2013 film Monsters") }
pub fn get_social_media_knowledge_74() -> String { String::from("University Disney Pixar created a Tumblr account MUGrumblr saying that the account is maintained by a Monstropolis transplant and self") }
pub fn get_social_media_knowledge_75() -> String { String::from("diagnosed coffee addict who is currently a sophomore at Monsters University A student from Monsters University uploaded memes animated GIFs") }
pub fn get_social_media_knowledge_76() -> String { String::from("and Instagram like photos related to the movie In 2014 Apple created a Tumblr page to promote the iPhone 5c") }
pub fn get_social_media_knowledge_77() -> String { String::from("labeling it Every color has a story with the website name ISee5c Upon opening the website the page is covered") }
pub fn get_social_media_knowledge_78() -> String { String::from("with different colors representing the iPhone 5c phone colors and case colors When a colored section is clicked a 15") }
pub fn get_social_media_knowledge_79() -> String { String::from("second video plays a song and showcases the dots featured on the rear of the iPhone 5c official cases and") }
pub fn get_social_media_knowledge_80() -> String { String::from("on the iOS 7 dynamic wallpapers concluding with words that are related to the video s theme Marketing techniques Social") }
pub fn get_social_media_knowledge_81() -> String { String::from("media marketing involves the use of social networks consumer s online brand related activities COBRA and electronic word of mouth") }
pub fn get_social_media_knowledge_82() -> String { String::from("to advertise online Social networks such as Facebook and Twitter provide advertisers with information about the likes and dislikes of") }
pub fn get_social_media_knowledge_83() -> String { String::from("their consumers This technique provides businesses with target audiences With social networks information relevant to users likes is available to") }
pub fn get_social_media_knowledge_84() -> String { String::from("businesses who then advertise accordingly Uploading pictures of one s own recently purchased products is an example of a consumer") }
pub fn get_social_media_knowledge_85() -> String { String::from("s online brand related activity Electronic recommendations and appraisals are a convenient manner to have a product promoted via consumer") }
pub fn get_social_media_knowledge_86() -> String { String::from("to consumer interactions An example of electronic word of mouth would be an online hotel review the hotel company can") }
pub fn get_social_media_knowledge_87() -> String { String::from("have two possible outcomes based on their service A good service would result in a positive review which gets the") }
pub fn get_social_media_knowledge_88() -> String { String::from("hotel free advertising via social media However a poor service will result in a negative consumer review which can potentially") }
pub fn get_social_media_knowledge_89() -> String { String::from("harm the company s reputation Social networking sites such as Facebook Instagram and Twitter have all influenced the buzz of") }
pub fn get_social_media_knowledge_90() -> String { String::from("word of mouth marketing In 1999 Misner said that word of mouth marketing is the world s most effective yet") }
pub fn get_social_media_knowledge_91() -> String { String::from("least understood marketing strategy Through the influence of opinion leaders the increased online buzz of word of mouth marketing that") }
pub fn get_social_media_knowledge_92() -> String { String::from("products services or companies experience is due to the rise in use of social media and smartphones Businesses and marketers") }
pub fn get_social_media_knowledge_93() -> String { String::from("have noticed that a person s behaviour is influenced by many small groups These small groups rotate around social networking") }
pub fn get_social_media_knowledge_94() -> String { String::from("accounts that are run by influential people opinion leaders or thought leaders who have followers of groups The types of") }
pub fn get_social_media_knowledge_95() -> String { String::from("groups followers are called Reference groups people who either know each other face to face or indirectly influence other people") }
pub fn get_social_media_knowledge_96() -> String { String::from("s attitudes or behaviours Membership groups people who directly influence other people s attitudes or behaviours Aspirational groups groups which") }
pub fn get_social_media_knowledge_97() -> String { String::from("an individual wishes to belong to Blogs Platforms like LinkedIn create an environment for companies and clients to connect online") }
pub fn get_social_media_knowledge_98() -> String { String::from("Companies that recognize the need for information originality and accessibility employ blogs to make their products popular and unique and") }
pub fn get_social_media_knowledge_99() -> String { String::from("ultimately reach out to consumers who are privy to social media Studies from 2009 show that consumers view coverage in") }
pub fn get_social_media_knowledge_100() -> String { String::from("the media or from bloggers as being more neutral and credible than print advertisements which are not thought of as") }
pub fn get_social_media_knowledge_101() -> String { String::from("free or independent Blogs allow a product or company to provide longer descriptions of products or services may include testimonials") }
pub fn get_social_media_knowledge_102() -> String { String::from("and links to related social media or blog content Blogs can be updated frequently and are promotional techniques for keeping") }
pub fn get_social_media_knowledge_103() -> String { String::from("customers and also for acquiring followers and subscribers who can then be directed to social network pages Online communities can") }
pub fn get_social_media_knowledge_104() -> String { String::from("enable a business to reach the clients of other businesses using the platform To allow firms to measure their standing") }
pub fn get_social_media_knowledge_105() -> String { String::from("in the corporate world sites enable employees to place evaluations of their companies Some businesses opt out of integrating social") }
pub fn get_social_media_knowledge_106() -> String { String::from("media platforms into their traditional marketing regimen There are also specific corporate standards that apply when interacting online To maintain") }
pub fn get_social_media_knowledge_107() -> String { String::from("an advantage in a business consumer relationship businesses have to be aware of four key assets that consumers maintain information") }
pub fn get_social_media_knowledge_108() -> String { String::from("involvement community and control Influencer marketing Marketers target influencers on social media that are recognized as being opinion leaders and") }
pub fn get_social_media_knowledge_109() -> String { String::from("opinion formers based on the credibility of their following An influencer s role under a brand sponsorship is to send") }
pub fn get_social_media_knowledge_110() -> String { String::from("messages to their target audiences through posts to amplify the credibility of a product or brand A social media post") }
pub fn get_social_media_knowledge_111() -> String { String::from("by an opinion leader can have a much greater impact via the forwarding or liking of the post than a") }
pub fn get_social_media_knowledge_112() -> String { String::from("social media post by a regular user Influencers can help brands obtain more consumers by promoting their products in an") }
pub fn get_social_media_knowledge_113() -> String { String::from("honest and genuine way using personal sales methods which is why brands consider collaborations with influencers to be a smart") }
pub fn get_social_media_knowledge_114() -> String { String::from("strategy However influencer marketing works well because it uses real shareable and viral content to reach a large audience and") }
pub fn get_social_media_knowledge_115() -> String { String::from("provide a profitable return on investment Marketers have realized that consumers are more prone to believe in other individuals who") }
pub fn get_social_media_knowledge_116() -> String { String::from("they trust Opinion leaders can also send their own messages about the products and services that they choose They have") }
pub fn get_social_media_knowledge_117() -> String { String::from("strong following bases because their opinions are valued or trusted Because of their personality beliefs values and other characteristics they") }
pub fn get_social_media_knowledge_118() -> String { String::from("have the potential to influence other people They usually have a large number of followers otherwise known as their reference") }
pub fn get_social_media_knowledge_119() -> String { String::from("membership or aspirational group An opinion leader s support of a product by posting a photo video or written recommendation") }
pub fn get_social_media_knowledge_120() -> String { String::from("on a blog can influence their followers and increase the chance of the brand selling more products or creating a") }
pub fn get_social_media_knowledge_121() -> String { String::from("following base of its own The adjusted communication model shows the use of using opinion leaders and opinion formers The") }
pub fn get_social_media_knowledge_122() -> String { String::from("sender source gives the message to many opinion leaders who pass the message on along with their personal opinions The") }
pub fn get_social_media_knowledge_123() -> String { String::from("receivers form their own opinions and send their personal messages to their friends and family Organic social media In contrast") }
pub fn get_social_media_knowledge_124() -> String { String::from("with pre Internet marketing such as TV ads and newspaper ads in which marketers controlled all aspects of their ads") }
pub fn get_social_media_knowledge_125() -> String { String::from("social media users are free to post comments right below online ads or post by companies about their products Companies") }
pub fn get_social_media_knowledge_126() -> String { String::from("are increasing the use of their social media strategies as part of their traditional marketing efforts via magazines newspapers radio") }
pub fn get_social_media_knowledge_127() -> String { String::from("advertisements and television advertisements Since the 2010s media consumers have often used multiple platforms at the same time e g") }
pub fn get_social_media_knowledge_128() -> String { String::from("surfing the Internet on a tablet while watching a streaming TV show so consistency of marketing content across all platforms") }
pub fn get_social_media_knowledge_129() -> String { String::from("has become necessary Heath 2006 wrote about the extent of attention that businesses should give to their social media sites") }
pub fn get_social_media_knowledge_130() -> String { String::from("It is about finding a balance between frequently posting but not over posting There is a lot more attention to") }
pub fn get_social_media_knowledge_131() -> String { String::from("be paid towards social media sites because people need updates to gain brand recognition Therefore a lot more content is") }
pub fn get_social_media_knowledge_132() -> String { String::from("needed and this can often be unplanned content Planned content begins with the creative marketing team generating their ideas Once") }
pub fn get_social_media_knowledge_133() -> String { String::from("they have completed their ideas they send them off for approval There are two general ways to do so The") }
pub fn get_social_media_knowledge_134() -> String { String::from("first is where each sector approves the plan one after another editor brand followed by the legal team Sectors may") }
pub fn get_social_media_knowledge_135() -> String { String::from("differ depending on the size and philosophy of the business The second is where each sector is given 24 hours") }
pub fn get_social_media_knowledge_136() -> String { String::from("or such designated time to sign off or disapprove If no action is given within the 24 hour period the") }
pub fn get_social_media_knowledge_137() -> String { String::from("original plan is implemented Planned content is often noticeable to customers and is un original or lacks excitement but is") }
pub fn get_social_media_knowledge_138() -> String { String::from("also a safer option to avoid unnecessary backlash from the public Both routes for planned content are time consuming as") }
pub fn get_social_media_knowledge_139() -> String { String::from("in the above on the first pathway to approval content takes 72 hours to be approved Although the second route") }
pub fn get_social_media_knowledge_140() -> String { String::from("can be significantly shorter it also holds more risk particularly in the legal department Unplanned content is an in the") }
pub fn get_social_media_knowledge_141() -> String { String::from("moment idea a spontaneous tactical reaction The content could be trending and not have the time to take the planned") }
pub fn get_social_media_knowledge_142() -> String { String::from("content route The unplanned content is posted sporadically and is not calendar date time arranged Deshpande 2014 Issues with unplanned") }
pub fn get_social_media_knowledge_143() -> String { String::from("content revolve around legal issues and whether the message being sent out represents the business brand accordingly If a company") }
pub fn get_social_media_knowledge_144() -> String { String::from("sends out a Tweet or Facebook message too hurriedly the company may unintentionally use insensitive language or messaging that could") }
pub fn get_social_media_knowledge_145() -> String { String::from("alienate some consumers For example celebrity chef Paula Deen was criticized after she made a social media post commenting about") }
pub fn get_social_media_knowledge_146() -> String { String::from("HIV AIDS and South Africa her message was deemed offensive by many observers The main difference between planned and unplanned") }
pub fn get_social_media_knowledge_147() -> String { String::from("is the time to approve the content Unplanned content must still be approved by marketing managers but in a much") }
pub fn get_social_media_knowledge_148() -> String { String::from("more rapid manner e g 1 2 hours or less Sectors may miss errors because of being hurried When using") }
pub fn get_social_media_knowledge_149() -> String { String::from("unplanned content Brito 2013 says be prepared to be reactive and respond to issues when they arise Brito writes about") }
pub fn get_social_media_knowledge_150() -> String { String::from("having a crisis escalation plan because It will happen The plan involves breaking down the issue into topics and classifying") }
pub fn get_social_media_knowledge_151() -> String { String::from("the issue into groups I dentify ing and flag ging potential risks also helps to organise an issue The problem") }
pub fn get_social_media_knowledge_152() -> String { String::from("can then be handled by the correct team and dissolved more effectively rather than any person at hand trying to") }
pub fn get_social_media_knowledge_153() -> String { String::from("solve the situation Platforms Individuals businesses and other organizations can interact with one another and build relationships and communities online") }
pub fn get_social_media_knowledge_154() -> String { String::from("through social networking websites Consumers can directly interact with companies that join these social channels These interactions can be more") }
pub fn get_social_media_knowledge_155() -> String { String::from("personal to users than traditional methods of outbound marketing and advertising The ability to rapidly change buying patterns and product") }
pub fn get_social_media_knowledge_156() -> String { String::from("or service acquisition and activity to a growing number of consumers is defined as an influence network On social networking") }
pub fn get_social_media_knowledge_157() -> String { String::from("sites and blogs users can repost comments made by others about a product being promoted which occurs quite frequently on") }
pub fn get_social_media_knowledge_158() -> String { String::from("some social media sites Users can extend the reach of messages by sharing them with their connections and bringing more") }
pub fn get_social_media_knowledge_159() -> String { String::from("traffic to products companies through word of mouth Businesses can interact with users directly and deliver targeted content based on") }
pub fn get_social_media_knowledge_160() -> String { String::from("user preferences By choosing whom to follow on these sites products can reach a very narrow target audience Social networking") }
pub fn get_social_media_knowledge_161() -> String { String::from("sites also host information about what products and services prospective clients might be interested in Marketers can use semantic analysis") }
pub fn get_social_media_knowledge_162() -> String { String::from("technologies to detect buying signals such as content shared by people and questions posted online An understanding of buying signals") }
pub fn get_social_media_knowledge_163() -> String { String::from("can help sales people target relevant prospects and marketers run micro targeted campaigns In 2014 over 80 of business executives") }
pub fn get_social_media_knowledge_164() -> String { String::from("identified social media as an integral part of their business Business retailers have seen 133 increases in their revenues from") }
pub fn get_social_media_knowledge_165() -> String { String::from("social media marketing Facebook Facebook pages are more detailed than Twitter accounts They allow a product to provide videos photos") }
pub fn get_social_media_knowledge_166() -> String { String::from("longer descriptions and testimonials where followers can comment on the product pages for others to see Facebook can link back") }
pub fn get_social_media_knowledge_167() -> String { String::from("to the product s Twitter page as well as send out event reminders As of May 2015 93 of businesses") }
pub fn get_social_media_knowledge_168() -> String { String::from("marketers used Facebook to promote their brand A study from 2011 attributed 84 of engagement or clicks and likes that") }
pub fn get_social_media_knowledge_169() -> String { String::from("link back to Facebook advertising By 2014 Facebook had restricted the content published from business and brands Adjustments in Facebook") }
pub fn get_social_media_knowledge_170() -> String { String::from("algorithms had reduced the audience for non paying business pages that have at least 500 000 likes from 16 in") }
pub fn get_social_media_knowledge_171() -> String { String::from("2012 down to 2 in February 2014 Instagram In May 2014 Instagram had over 200 million users The user engagement") }
pub fn get_social_media_knowledge_172() -> String { String::from("rate of Instagram was 15 times higher than of Facebook and 25 times higher than that of Twitter LinkedIn Companies") }
pub fn get_social_media_knowledge_173() -> String { String::from("can create professional LinkedIn profiles for themselves and their business to network and meet others LinkedIn members can use Company") }
pub fn get_social_media_knowledge_174() -> String { String::from("Pages similar to Facebook pages to create an area on which business owners can promote their products or services and") }
pub fn get_social_media_knowledge_175() -> String { String::from("interact with their customers Snapchat Snapchat is an American multimedia instant messaging app One of the principal features of Snapchat") }
pub fn get_social_media_knowledge_176() -> String { String::from("is that pictures and messages are usually available for only a short time before they become inaccessible to their recipients") }
pub fn get_social_media_knowledge_177() -> String { String::from("TikTok TikTok was first released in 2016 and became one of the most popular social media apps with over 1") }
pub fn get_social_media_knowledge_178() -> String { String::from("billion users worldwide It is mainly mobile based and allows users to post short video content Tumblr Blogging website Tumblr") }
pub fn get_social_media_knowledge_179() -> String { String::from("first launched ad products on May 29 2012 Rather than relying on simple banner ads Tumblr requires advertisers to create") }
pub fn get_social_media_knowledge_180() -> String { String::from("a Tumblr blog so the content of those blogs can be featured on the site In one year four native") }
pub fn get_social_media_knowledge_181() -> String { String::from("ad formats were created on web and mobile and had more than 100 brands advertising on Tumblr with 500 cumulative") }
pub fn get_social_media_knowledge_182() -> String { String::from("sponsored posts Twitter X Twitter allows companies to promote their products in short messages known as tweets limited to 280") }
pub fn get_social_media_knowledge_183() -> String { String::from("characters which appear on followers Home timelines Twitter has also been used by companies to provide customer service Yelp Yelp") }
pub fn get_social_media_knowledge_184() -> String { String::from("consists of a comprehensive online index of business profiles Businesses are searchable by location similar to Yellow Pages The website") }
pub fn get_social_media_knowledge_185() -> String { String::from("is operational in seven different countries including the United States and Canada Business account holders can create share and edit") }
pub fn get_social_media_knowledge_186() -> String { String::from("business profiles and post information such as their business s location and contact information along with pictures and service information") }
pub fn get_social_media_knowledge_187() -> String { String::from("Individuals can write post reviews about businesses and rate them on a five point scale Messaging and talk features are") }
pub fn get_social_media_knowledge_188() -> String { String::from("additionally made available for general members of the website serving to guide thoughts and opinions YouTube Advertisements on YouTube can") }
pub fn get_social_media_knowledge_189() -> String { String::from("use targeted advertising via Google Ads Advertisers can also sponsor videos directly which is a form of native advertising YouTube") }
pub fn get_social_media_knowledge_190() -> String { String::from("also enable publishers to earn money through its YouTube Partner Program Companies can pay YouTube for a special channel which") }
pub fn get_social_media_knowledge_191() -> String { String::from("promotes the companies products or services Social bookmarking sites Social bookmarking sites are used in social media promotion Each of") }
pub fn get_social_media_knowledge_192() -> String { String::from("these sites is dedicated to the collection curation and organization of links to other websites that users deem to be") }
pub fn get_social_media_knowledge_193() -> String { String::from("of good quality This process is crowdsourced allowing amateur social media network members to sort and prioritize links by relevance") }
pub fn get_social_media_knowledge_194() -> String { String::from("and general category Due to the large user bases of these websites any link from one of them to another") }
pub fn get_social_media_knowledge_195() -> String { String::from("the smaller website may in a flash crowd a sudden surge of interest in the target website In addition to") }
pub fn get_social_media_knowledge_196() -> String { String::from("user generated promotion these sites also offer advertisements within individual user communities and categories Because ads can be placed in") }
pub fn get_social_media_knowledge_197() -> String { String::from("designated communities with a very specific target audience and demographic they have far greater potential for traffic generation than ads") }
pub fn get_social_media_knowledge_198() -> String { String::from("selected simply through cookie and browser history Additionally some of these websites have also implemented measures to make ads more") }
pub fn get_social_media_knowledge_199() -> String { String::from("relevant to users by allowing users to vote on which ones will be shown on pages they frequent The ability") }
pub fn get_social_media_knowledge_200() -> String { String::from("to redirect large volumes of web traffic and target specific relevant audiences makes social bookmarking sites a valuable asset for") }
pub fn get_social_media_knowledge_201() -> String { String::from("social media marketers Implications on traditional advertising Minimizing use Traditional advertising techniques include print and television advertising The Internet has") }
pub fn get_social_media_knowledge_202() -> String { String::from("already overtaken television as the largest advertising market Web sites often include banner or pop up ads Social networking sites") }
pub fn get_social_media_knowledge_203() -> String { String::from("do not always carry ads In exchange products have entire pages and are able to interact with users Television commercials") }
pub fn get_social_media_knowledge_204() -> String { String::from("often end with a spokesperson asking viewers to check out the product website for more information While briefly popular print") }
pub fn get_social_media_knowledge_205() -> String { String::from("ads included QR codes which can be scanned by cell phones and computers sending viewers to the product website Advertising") }
pub fn get_social_media_knowledge_206() -> String { String::from("is beginning to move viewers from traditional outlets to electronic ones Mishaps Due to the viral nature of the Internet") }
pub fn get_social_media_knowledge_207() -> String { String::from("mishaps with social media posts can be damaging for organizations In 2011 designer Kenneth Cole tweeted Millions are in uproar") }
pub fn get_social_media_knowledge_208() -> String { String::from("in Cairo Rumor has they heard our new spring collection is now available online at Kenneth Cole s website This") }
pub fn get_social_media_knowledge_209() -> String { String::from("reference to the 2011 Egyptian revolution drew an objection from the public it was widely objected to on the Internet") }
pub fn get_social_media_knowledge_210() -> String { String::from("Kenneth Cole realized his mistake shortly after and responded with a statement apologizing for the tweet In 2011 a Chrysler") }
pub fn get_social_media_knowledge_211() -> String { String::from("Group employee tweeted that no one in Detroit knows how to drive In 2012 during Hurricane Sandy Gap sent out") }
pub fn get_social_media_knowledge_212() -> String { String::from("a tweet to its followers telling them to stay safe but encouraged them to shop online and offered free shipping") }
pub fn get_social_media_knowledge_213() -> String { String::from("The tweet was deemed insensitive and Gap eventually took it down and apologized When the Link REIT opened a Facebook") }
pub fn get_social_media_knowledge_214() -> String { String::from("page to recommend old style restaurants the page was flooded by furious comments criticizing the REIT for having forced a") }
pub fn get_social_media_knowledge_215() -> String { String::from("lot of restaurants and stores to shut down it had to terminate its campaign early amid further deterioration of its") }
pub fn get_social_media_knowledge_216() -> String { String::from("corporate image In 2018 Max Factor MAC and other beauty brands were forced to rush to disassociate themselves from Kuwaiti") }
pub fn get_social_media_knowledge_217() -> String { String::from("beauty blogger and Instagram influencer Sondos Alqattan after she criticised government moves to improve conditions for domestic workers Ethics The") }
pub fn get_social_media_knowledge_218() -> String { String::from("ethical principles associated with traditional marketing can also be applied to social media However with social media being so personal") }
pub fn get_social_media_knowledge_219() -> String { String::from("and international online ethics come with additional complications and challenges A sensitive topic amongst social media professionals is the subject") }
pub fn get_social_media_knowledge_220() -> String { String::from("of ethics in social media marketing practices specifically the proper uses of often very personal data With social media marketers") }
pub fn get_social_media_knowledge_221() -> String { String::from("can see what consumers like to hear from advertisers how they engage online and what their needs and wants are") }
pub fn get_social_media_knowledge_222() -> String { String::from("instead of focusing solely on the basic demographics and psychographics given from television and magazines The general concept of ethical") }
pub fn get_social_media_knowledge_223() -> String { String::from("social media usage entails honesty with a campaign s intentions avoiding false advertising awareness of user privacy conditions which means") }
pub fn get_social_media_knowledge_224() -> String { String::from("not using consumers private information for gain respecting people s dignity and taking responsibility for mistakes or mishaps that result") }
pub fn get_social_media_knowledge_225() -> String { String::from("from marketing campaigns Most social network marketers use websites like Facebook and MySpace to try to drive traffic to another") }
pub fn get_social_media_knowledge_226() -> String { String::from("website In addition social media platforms have become extremely aware of their users and collect information about their viewers to") }
pub fn get_social_media_knowledge_227() -> String { String::from("connect with them in various ways Facebook is quietly working on a new advertising system that would let marketers target") }
pub fn get_social_media_knowledge_228() -> String { String::from("users with ads based on the massive amounts of information people reveal on the site about themselves Some people may") }
pub fn get_social_media_knowledge_229() -> String { String::from("react negatively because they believe it is an invasion of privacy On the other hand some individuals may enjoy this") }
pub fn get_social_media_knowledge_230() -> String { String::from("feature because their social network recognizes their interests and sends them particular advertisements pertaining to those interests Consumers like to") }
pub fn get_social_media_knowledge_231() -> String { String::from("network with people who share their interests and desires Managers invest in social media to foster relationships and interact with") }
pub fn get_social_media_knowledge_232() -> String { String::from("customers For many users data collection is a breach of privacy but there are no laws that prevent these companies") }
pub fn get_social_media_knowledge_233() -> String { String::from("from using the information provided on their websites Companies like Equifax TransUnion and LexisNexis thrive on collecting and sharing the") }
pub fn get_social_media_knowledge_234() -> String { String::from("personal information of social media users In 2012 Facebook purchased information about 70 million households from a third party company") }
pub fn get_social_media_knowledge_235() -> String { String::from("called Datalogix Facebook later revealed that they purchased the information in order to create a more efficient advertising service See") }
pub fn get_social_media_knowledge_236() -> String { String::from("also Integrated marketing communications Internet marketing Social media in the fashion industry Social media optimization Social media spam Social video") }
pub fn get_social_media_knowledge_237() -> String { String::from("marketing Visual marketing Web 2 0 internet celebrity Social media analytics References External links Generation Like Frontline Season 32 Episode") }
pub fn get_social_media_knowledge_238() -> String { String::from("4 February 18 2014 PBS WGBH Retrieved August 16 2025 Bria Francesca 2014 Social media and their impact on organisations") }
pub fn get_social_media_knowledge_239() -> String { String::from("building Firm Celebrity and organisational legitimacy through social media Archived 2020 02 23 at the Wayback Machine dissertation Retrieved 13") }
pub fn get_social_media_knowledge_240() -> String { String::from("September 2018 Kang Juhee 2015 Social media marketing dissertation Journal of Marketing Retrieved 8 February 2015 Digital marketing is a") }
pub fn get_social_media_knowledge_241() -> String { String::from("component of marketing that uses digital technologies such as desktop computers mobile phones and other digital media platforms to promote") }
pub fn get_social_media_knowledge_242() -> String { String::from("products and services It has significantly transformed how brands and businesses use technology for marketing since the 1990s and 2000s") }
pub fn get_social_media_knowledge_243() -> String { String::from("As digital platforms became increasingly incorporated into marketing plans and everyday life and as people increasingly used digital devices instead") }
pub fn get_social_media_knowledge_244() -> String { String::from("of visiting physical shops digital marketing campaigns have become increasingly prevalent employing combinations of methods These methods include search engine") }
pub fn get_social_media_knowledge_245() -> String { String::from("optimization SEO search engine marketing SEM content marketing influencer marketing content automation campaign marketing data driven marketing e commerce marketing") }
pub fn get_social_media_knowledge_246() -> String { String::from("social media marketing social media optimization e mail direct marketing display advertising e books optical disks and games Digital marketing") }
pub fn get_social_media_knowledge_247() -> String { String::from("also extends to non Internet channels that provide digital media such as television mobile phones SMS and MMS callbacks and") }
pub fn get_social_media_knowledge_248() -> String { String::from("on hold mobile ringtones The extension to non Internet channels differentiates digital marketing from online marketing History Digital marketing effectively") }
pub fn get_social_media_knowledge_249() -> String { String::from("began in 1990 when the Archie search engine was created as an index for FTP sites During the 1980s the") }
pub fn get_social_media_knowledge_250() -> String { String::from("storage capacity of computers had already large enough to store large volumes of customer information As a result companies started") }
pub fn get_social_media_knowledge_251() -> String { String::from("choosing online techniques such as database marketing rather than relying on limited list brokers These databases allowed companies to track") }
pub fn get_social_media_knowledge_252() -> String { String::from("customers information more effectively transforming the relationship between buyers and sellers In the 1990s the term digital marketing was coined") }
pub fn get_social_media_knowledge_253() -> String { String::from("The first clickable banner ad the You Will campaign by AT T launched in 1994 Within the first four months") }
pub fn get_social_media_knowledge_254() -> String { String::from("44 of all people who viewed the advertisement clicked on it Early digital marketing efforts focused primarily on simple HTML") }
pub fn get_social_media_knowledge_255() -> String { String::from("websites and the burgeoning practice of email marketing which enabled direct communication with consumers In the 2000s with increasing numbers") }
pub fn get_social_media_knowledge_256() -> String { String::from("of Internet users and the introduction of the iPhone customers began searching for products and making decisions online instead of") }
pub fn get_social_media_knowledge_257() -> String { String::from("consulting a salesperson which created a new problem for the marketing department of a company Additionally a survey conducted in") }
pub fn get_social_media_knowledge_258() -> String { String::from("the United Kingdom in 2000 revealed that most retailers had not yet registered their own domain names These challenges encouraged") }
pub fn get_social_media_knowledge_259() -> String { String::from("marketers to explore new ways to integrate digital technology into marketing strategies At the same time pay per click PPC") }
pub fn get_social_media_knowledge_260() -> String { String::from("advertising introduced by Google AdWords in 2000 allowed businesses to target specific keywords making digital marketing more measurable and cost") }
pub fn get_social_media_knowledge_261() -> String { String::from("effective The mid 2000s saw the emergence of social media platforms such as Facebook 2004 YouTube 2005 and Twitter 2006") }
pub fn get_social_media_knowledge_262() -> String { String::from("These platforms revolutionized digital marketing by facilitating direct and interactive engagement with consumers In 2007 marketing automation was introduced as") }
pub fn get_social_media_knowledge_263() -> String { String::from("a response to the rapidly evolving marketing climate Marketing automation is the process by which software is used to automate") }
pub fn get_social_media_knowledge_264() -> String { String::from("conventional marketing processes Marketing automation helps companies to segment customers launch multichannel marketing campaigns and provide personalized information for customers") }
pub fn get_social_media_knowledge_265() -> String { String::from("based on their specific activities In this way users activity or lack thereof triggers a personal message that is customized") }
pub fn get_social_media_knowledge_266() -> String { String::from("to the user in their preferred platform However despite the benefits of marketing automation many companies are struggling to adapt") }
pub fn get_social_media_knowledge_267() -> String { String::from("it to their everyday uses correctly Digital marketing became increasingly sophisticated during 2000s and 2010s when the proliferation of devices") }
pub fn get_social_media_knowledge_268() -> String { String::from("capable of accessing digital media led to sudden growth Statistics produced in 2012 and 2013 showed that digital marketing continued") }
pub fn get_social_media_knowledge_269() -> String { String::from("to grow significantly With the development of social media in the 2000s such as LinkedIn Facebook YouTube and Twitter consumers") }
pub fn get_social_media_knowledge_270() -> String { String::from("became highly dependent on digital electronics in their daily lives Therefore they expected a seamless user experience across different channels") }
pub fn get_social_media_knowledge_271() -> String { String::from("for searching product information The change in customer behavior improved the diversification of marketing technology Digital media growth was estimated") }
pub fn get_social_media_knowledge_272() -> String { String::from("at 4 5 trillion online ads served annually with digital media spending at 48 growth in 2010 An increasing portion") }
pub fn get_social_media_knowledge_273() -> String { String::from("of advertising stems from businesses employing Online Behavioural Advertising OBA to tailor advertising for internet users but OBA raises concerns") }
pub fn get_social_media_knowledge_274() -> String { String::from("about consumer privacy and data protection In the 2020s the rise of generative artificial intelligence tools such as ChatGPT Claude") }
pub fn get_social_media_knowledge_275() -> String { String::from("Perplexity AI and Gemini changed how people find information online with AI generated responses increasingly competing with traditional search results") }
pub fn get_social_media_knowledge_276() -> String { String::from("As a result digital marketers adopted new methods for optimizing content for large language model LLM based search systems These") }
pub fn get_social_media_knowledge_277() -> String { String::from("methods became known as generative engine optimization GEO or answer engine optimization AEO GEO methods documented in the academic literature") }
pub fn get_social_media_knowledge_278() -> String { String::from("include adding citations and quotations from credible sources incorporating relevant statistics adopting a more authoritative writing style and improving the") }
pub fn get_social_media_knowledge_279() -> String { String::from("fluency of website content Brand awareness One of the key objectives of modern digital marketing is to raise brand awareness") }
pub fn get_social_media_knowledge_280() -> String { String::from("the extent to which customers and the public are familiar with and recognize a particular brand Enhancing brand awareness is") }
pub fn get_social_media_knowledge_281() -> String { String::from("important in digital marketing and marketing in general because of its impact on brand perception and consumer decision making Channels") }
pub fn get_social_media_knowledge_282() -> String { String::from("Digital Marketing Channels are systems based on the Internet that can create accelerate and transmit product value from producer to") }
pub fn get_social_media_knowledge_283() -> String { String::from("a consumer terminal through digital networks Digital marketing is facilitated by multiple Digital Marketing channels as an advertiser one s") }
pub fn get_social_media_knowledge_284() -> String { String::from("core objective is to find channels which result in maximum two way communication and a better overall ROI for the") }
pub fn get_social_media_knowledge_285() -> String { String::from("brand There are multiple digital marketing channels available namely Affiliate marketing Affiliate marketing is perceived to not be considered a") }
pub fn get_social_media_knowledge_286() -> String { String::from("safe reliable and easy means of marketing through online platforms This is due to a lack of reliability in terms") }
pub fn get_social_media_knowledge_287() -> String { String::from("of affiliates that can produce the demanded number of new customers As a result of this risk and bad affiliates") }
pub fn get_social_media_knowledge_288() -> String { String::from("it leaves the brand prone to exploitation in terms of claiming commission that isn t honestly acquired Legal means may") }
pub fn get_social_media_knowledge_289() -> String { String::from("offer some protection against this yet there are limitations in recovering any losses or investment Despite this affiliate marketing allows") }
pub fn get_social_media_knowledge_290() -> String { String::from("the brand to market towards smaller publishers and websites with smaller traffic Brands that choose to use this marketing often") }
pub fn get_social_media_knowledge_291() -> String { String::from("should beware of such risks involved and look to associate with affiliates in which rules are laid down between the") }
pub fn get_social_media_knowledge_292() -> String { String::from("parties involved to assure and minimize the risk involved Display advertising As the term implies online display advertising deals with") }
pub fn get_social_media_knowledge_293() -> String { String::from("showcasing promotional messages or ideas to the consumer on the internet This includes a wide range of advertisements like advertising") }
pub fn get_social_media_knowledge_294() -> String { String::from("blogs networks interstitial ads contextual data ads on search engines classified or dynamic advertisements etc The method can target specific") }
pub fn get_social_media_knowledge_295() -> String { String::from("audience tuning in from different types of locals to view a particular advertisement the variations can be found as the") }
pub fn get_social_media_knowledge_296() -> String { String::from("most productive element of this method Email marketing Email marketing in comparison to other forms of digital marketing is considered") }
pub fn get_social_media_knowledge_297() -> String { String::from("cheap It is also a way to rapidly communicate a message such as their value proposition to existing or potential") }
pub fn get_social_media_knowledge_298() -> String { String::from("customers Yet this channel of communication may be perceived by recipients to be bothersome and irritating especially to new or") }
pub fn get_social_media_knowledge_299() -> String { String::from("potential customers therefore the success of email marketing is reliant on the language and visual appeal applied In terms of") }
pub fn get_social_media_knowledge_300() -> String { String::from("visual appeal there are indications that using graphics visuals that are relevant to the message which is attempting to be") }
pub fn get_social_media_knowledge_301() -> String { String::from("sent yet less visual graphics to be applied with initial emails are more effective in turn creating a relatively personal") }
pub fn get_social_media_knowledge_302() -> String { String::from("feel to the email In terms of language the style is the main factor in determining how captivating the email") }
pub fn get_social_media_knowledge_303() -> String { String::from("is Using a casual tone invokes a warmer gentler and more inviting feel to the email compared to a more") }
pub fn get_social_media_knowledge_304() -> String { String::from("formal tone Search engine marketing Search engine marketing SEM is a form of Internet marketing that involves the promotion of") }
pub fn get_social_media_knowledge_305() -> String { String::from("websites by increasing their visibility in search engine results pages SERPs primarily through paid advertising SEM may incorporate Search engine") }
pub fn get_social_media_knowledge_306() -> String { String::from("optimization which adjusts or rewrites website content and site architecture to achieve a higher ranking in search engine results pages") }
pub fn get_social_media_knowledge_307() -> String { String::from("to enhance pay per click PPC listings Social Media Marketing The term Digital Marketing has a number of marketing facets") }
pub fn get_social_media_knowledge_308() -> String { String::from("as it supports different channels used in and among these comes the Social Media When we use social media channels") }
pub fn get_social_media_knowledge_309() -> String { String::from("Facebook Twitter Pinterest Instagram Google etc to market a product or service the strategy is called Social Media Marketing It") }
pub fn get_social_media_knowledge_310() -> String { String::from("is a procedure wherein strategies are made and executed to draw in traffic for a website or to gain the") }
pub fn get_social_media_knowledge_311() -> String { String::from("attention of buyers over the web using different social media platforms Social networking service A social networking service is an") }
pub fn get_social_media_knowledge_312() -> String { String::from("online platform which people use to build social networks or social relations with other people who share similar personal or") }
pub fn get_social_media_knowledge_313() -> String { String::from("career interests activities backgrounds or real life connections In game advertising In Game advertising is defined as the inclusion of") }
pub fn get_social_media_knowledge_314() -> String { String::from("products or brands within a digital game The game allows brands or products to place ads within their game either") }
pub fn get_social_media_knowledge_315() -> String { String::from("in a subtle manner or in the form of an advertisement banner There are many factors that exist in whether") }
pub fn get_social_media_knowledge_316() -> String { String::from("brands are successful in the advertising of their brand product these being Type of game technical platform 3 D and") }
pub fn get_social_media_knowledge_317() -> String { String::from("4 D technology game genre congruity of brand and game prominence of advertising within the game Individual factors consist of") }
pub fn get_social_media_knowledge_318() -> String { String::from("attitudes towards placement advertisements game involvement product involvement flow or entertainment The attitude towards the advertising also takes into account") }
pub fn get_social_media_knowledge_319() -> String { String::from("not only the message shown but also the attitude towards the game Dependent on how enjoyable the game is will") }
pub fn get_social_media_knowledge_320() -> String { String::from("determine how the brand is perceived meaning if the game isn t very enjoyable the consumer may subconsciously have a") }
pub fn get_social_media_knowledge_321() -> String { String::from("negative attitude towards the brand product being advertised In terms of Integrated Marketing Communication integration of advertising in digital games") }
pub fn get_social_media_knowledge_322() -> String { String::from("into the general advertising communication and marketing strategy of the firm is important as it results in a more clarity") }
pub fn get_social_media_knowledge_323() -> String { String::from("about the brand product and creates a larger overall effect Online public relations The use of the internet to communicate") }
pub fn get_social_media_knowledge_324() -> String { String::from("with both potential and current customers in the public realm Video advertising This type of advertising in terms of digital") }
pub fn get_social_media_knowledge_325() -> String { String::from("online means are advertisements that play on online videos e g YouTube videos This type of marketing has seen an") }
pub fn get_social_media_knowledge_326() -> String { String::from("increase in popularity over time Online Video Advertising usually consists of three types Pre Roll advertisements which play before the") }
pub fn get_social_media_knowledge_327() -> String { String::from("video is watched Mid Roll advertisements which play during the video or Post Roll advertisements which play after the video") }
pub fn get_social_media_knowledge_328() -> String { String::from("is watched Post roll advertisements were shown to have better brand recognition in relation to the other types where as") }
pub fn get_social_media_knowledge_329() -> String { String::from("ad context congruity incongruity plays an important role in reinforcing ad memorability Due to selective attention from viewers there is") }
pub fn get_social_media_knowledge_330() -> String { String::from("the likelihood that the message may not be received The main advantage of video advertising is that it disrupts the") }
pub fn get_social_media_knowledge_331() -> String { String::from("viewing experience of the video and therefore there is a difficulty in attempting to avoid them How a consumer interacts") }
pub fn get_social_media_knowledge_332() -> String { String::from("with online video advertising can come down to three stages Pre attention attention and behavioral decision These online advertisements give") }
pub fn get_social_media_knowledge_333() -> String { String::from("the brand business options and choices These consist of length position adjacent video content which all directly affect the effectiveness") }
pub fn get_social_media_knowledge_334() -> String { String::from("of the produced advertisement time therefore manipulating these variables will yield different results The length of the advertisement has shown") }
pub fn get_social_media_knowledge_335() -> String { String::from("to affect memorability where as a longer duration resulted in increased brand recognition This type of advertising due to its") }
pub fn get_social_media_knowledge_336() -> String { String::from("nature of interruption of the viewer it is likely that the consumer may feel as if their experience is being") }
pub fn get_social_media_knowledge_337() -> String { String::from("interrupted or invaded creating negative perception of the brand These advertisements are also available to be shared by the viewers") }
pub fn get_social_media_knowledge_338() -> String { String::from("adding to the attractiveness of this platform Sharing these videos can be equated to the online version of word by") }
pub fn get_social_media_knowledge_339() -> String { String::from("mouth marketing extending number of people reached Sharing videos creates six different outcomes these being pleasure affection inclusion escape relaxation") }
pub fn get_social_media_knowledge_340() -> String { String::from("and control As well videos that have entertainment value are more likely to be shared yet pleasure is the strongest") }
pub fn get_social_media_knowledge_341() -> String { String::from("motivator to pass videos on Creating a viral trend from a mass amount of a brand advertisement can maximize the") }
pub fn get_social_media_knowledge_342() -> String { String::from("outcome of an online video advert whether it be positive or a negative outcome Native Advertising This involves the placement") }
pub fn get_social_media_knowledge_343() -> String { String::from("of paid content that replicates the look feel and oftentimes the voice of a platform s existing content It is") }
pub fn get_social_media_knowledge_344() -> String { String::from("most effective when used on digital platforms like websites newsletters and social media Can be somewhat controversial as some critics") }
pub fn get_social_media_knowledge_345() -> String { String::from("feel it intentionally deceives consumers Content Marketing This is an approach to marketing that focuses on gaining and retaining customers") }
pub fn get_social_media_knowledge_346() -> String { String::from("by offering helpful content to customers that improves the buying experience and creates brand awareness A brand may use this") }
pub fn get_social_media_knowledge_347() -> String { String::from("approach to hold a customer s attention with the goal of influencing potential purchase decisions Sponsored Content This utilises content") }
pub fn get_social_media_knowledge_348() -> String { String::from("created and paid for by a brand to promote a specific product or service Inbound Marketing a market strategy that") }
pub fn get_social_media_knowledge_349() -> String { String::from("involves using content as a means to attract customers to a brand or product Requires extensive research into the behaviors") }
pub fn get_social_media_knowledge_350() -> String { String::from("interests and habits of the brand s target market SMS Marketing Although the popularity is decreasing day by day still") }
pub fn get_social_media_knowledge_351() -> String { String::from("SMS marketing plays a large role in bringing new users providing direct updates providing new offers etc Push Notification Push") }
pub fn get_social_media_knowledge_352() -> String { String::from("notifications are responsible for bringing new and abandoned customers through smart segmentation Many online brands are using this to provide") }
pub fn get_social_media_knowledge_353() -> String { String::from("personalised appeals depending on the scenario of customer acquisition It is important for a firm to reach out to consumers") }
pub fn get_social_media_knowledge_354() -> String { String::from("and create a two way communication model as digital marketing allows consumers to give back feedback to the firm on") }
pub fn get_social_media_knowledge_355() -> String { String::from("a community based site or straight directly to the firm via email Firms should seek this long term communication relationship") }
pub fn get_social_media_knowledge_356() -> String { String::from("by using multiple forms of channels and using promotional strategies related to their target consumer as well as word of") }
pub fn get_social_media_knowledge_357() -> String { String::from("mouth marketing Regulation Digital marketing used to rely primarily on self regulation included in the ICC Code which included rules") }
pub fn get_social_media_knowledge_358() -> String { String::from("that apply to marketing communications using digital interactive media However self regulation has proved largely ineffective leading to the consolidation") }
pub fn get_social_media_knowledge_359() -> String { String::from("of market power in a few firms including Google which has been determined to hold monopolies in search marketing and") }
pub fn get_social_media_knowledge_360() -> String { String::from("digital advertising While self regulation codes still exist government regulation is increasing in multiple jurisdictions including California s legislation on") }
pub fn get_social_media_knowledge_361() -> String { String::from("targeting advertising online In Europe digital marketing is regulated through multiple codes of which the most important is the Digital") }
pub fn get_social_media_knowledge_362() -> String { String::from("Services Act which entered into force on 17 February 2024 Other regulations focus on user privacy and data management such") }
pub fn get_social_media_knowledge_363() -> String { String::from("as the General Data Protection Regulation GDPR Strategy Planning Digital marketing planning is a term used in marketing management It") }
pub fn get_social_media_knowledge_364() -> String { String::from("describes the first stage of forming a digital marketing strategy for the wider digital marketing system The difference between digital") }
pub fn get_social_media_knowledge_365() -> String { String::from("and traditional marketing planning is that it uses digitally based communication tools and technology such as Social Web Mobile Scannable") }
pub fn get_social_media_knowledge_366() -> String { String::from("Surface Nevertheless both are aligned with the vision the mission of the company and the overarching business strategy Stages of") }
pub fn get_social_media_knowledge_367() -> String { String::from("planning Dr Dave Chaffey an author on marketing topics has suggested that successful digital marketing strategies have do digital marketing") }
pub fn get_social_media_knowledge_368() -> String { String::from("planning DMP which is a three stage approach Opportunity Strategy and Action This generic strategic approach often has phases of") }
pub fn get_social_media_knowledge_369() -> String { String::from("situation review goal setting strategy formulation resource allocation and monitoring Opportunity To create an effective DMP a business first needs") }
pub fn get_social_media_knowledge_370() -> String { String::from("to review the marketplace and set SMART specific measurable actionable relevant and time bound objectives They can set SMART objectives") }
pub fn get_social_media_knowledge_371() -> String { String::from("by reviewing the current benchmarks and key performance indicators KPIs of the company and competitors It is pertinent that the") }
pub fn get_social_media_knowledge_372() -> String { String::from("analytics used for the KPIs be customized to the type objectives mission and vision of the company Companies can scan") }
pub fn get_social_media_knowledge_373() -> String { String::from("for marketing and sales opportunities by reviewing their own outreach as well as influencer outreach This means they have competitive") }
pub fn get_social_media_knowledge_374() -> String { String::from("advantage because they are able to analyse their co marketers influence and brand associations To seize the opportunity the firm") }
pub fn get_social_media_knowledge_375() -> String { String::from("should summarize its current customers personas and purchase journey from this they are able to deduce their digital marketing capability") }
pub fn get_social_media_knowledge_376() -> String { String::from("Strategy A planned digital strategy is where a company expresses clearly what they are offering customers online e g brand") }
pub fn get_social_media_knowledge_377() -> String { String::from("positioning The marketing mix is a framework which can be used to facilitate this Action The third and final stage") }
pub fn get_social_media_knowledge_378() -> String { String::from("requires the firm to set a budget and management systems These must be measurable touchpoints such as the audience reached") }
pub fn get_social_media_knowledge_379() -> String { String::from("across all digital platforms Furthermore marketers must ensure the budget and management systems are integrating the paid owned and earned") }
pub fn get_social_media_knowledge_380() -> String { String::from("media of the company The Action and final stage of planning also requires the company to set in place measurable") }
pub fn get_social_media_knowledge_381() -> String { String::from("content creation e g oral visual or written online media Understanding the market One way marketers can reach out to") }
pub fn get_social_media_knowledge_382() -> String { String::from("consumers and understand their thought process is through what is called an empathy map An empathy map is a four") }
pub fn get_social_media_knowledge_383() -> String { String::from("step process The first step is through asking questions that the consumer would be thinking in their demographic The second") }
pub fn get_social_media_knowledge_384() -> String { String::from("step is to describe the feelings that the consumer may be having The third step is to think about what") }
pub fn get_social_media_knowledge_385() -> String { String::from("the consumer would say in their situation The final step is to imagine what the consumer will try to do") }
pub fn get_social_media_knowledge_386() -> String { String::from("based on the other three steps This map is so marketing teams can put themselves in their target demographics shoes") }
pub fn get_social_media_knowledge_387() -> String { String::from("Web Analytics are also a very important way to understand consumers They show the habits that people have online for") }
pub fn get_social_media_knowledge_388() -> String { String::from("each website One particular form of these analytics is predictive analytics which helps marketers figure out what route consumers are") }
pub fn get_social_media_knowledge_389() -> String { String::from("on This uses the information gathered from other analytics and then creates different predictions of what people will do so") }
pub fn get_social_media_knowledge_390() -> String { String::from("that companies can strategize on what to do next according to the people s trends Sharing economy The sharing economy") }
pub fn get_social_media_knowledge_391() -> String { String::from("refers to an economic pattern that aims to obtain a resource that is not fully used Nowadays the sharing economy") }
pub fn get_social_media_knowledge_392() -> String { String::from("has had an unimagined effect on many traditional elements including labor industry and distribution system This effect is not negligible") }
pub fn get_social_media_knowledge_393() -> String { String::from("that some industries are obviously under threat The sharing economy is influencing the traditional marketing channels by changing the nature") }
pub fn get_social_media_knowledge_394() -> String { String::from("of some specific concept including ownership assets and recruitment Digital marketing channels and traditional marketing channels are similar in function") }
pub fn get_social_media_knowledge_395() -> String { String::from("that the value of the product or service is passed from the original producer to the end user by a") }
pub fn get_social_media_knowledge_396() -> String { String::from("kind of supply chain Digital Marketing channels however consist of internet systems that create promote and deliver products or services") }
pub fn get_social_media_knowledge_397() -> String { String::from("from producer to consumer through digital networks Increasing changes to marketing channels has been a significant contributor to the expansion") }
pub fn get_social_media_knowledge_398() -> String { String::from("and growth of the sharing economy Such changes to marketing channels has prompted unprecedented and historic growth In addition to") }
pub fn get_social_media_knowledge_399() -> String { String::from("this typical approach the built in control efficiency and low cost of digital marketing channels is an essential features in") }
pub fn get_social_media_knowledge_400() -> String { String::from("the application of sharing economy Digital marketing channels within the sharing economy are typically divided into three domains including e") }
pub fn get_social_media_knowledge_401() -> String { String::from("mail social media and search engine marketing or SEM E mail A form of direct marketing characterized as being informative") }
pub fn get_social_media_knowledge_402() -> String { String::from("promotional and often a means of customer relationship management Organization can update the activity or promotion information to the user") }
pub fn get_social_media_knowledge_403() -> String { String::from("by subscribing the newsletter mail that happened in consuming Success is reliant upon a company s ability to access contact") }
pub fn get_social_media_knowledge_404() -> String { String::from("information from its past present and future clientele Social media Social media has the capability to reach a larger audience") }
pub fn get_social_media_knowledge_405() -> String { String::from("in a shorter time frame than traditional marketing channels This makes social media a powerful tool for consumer engagement and") }
pub fn get_social_media_knowledge_406() -> String { String::from("the dissemination of information Search engine marketing SEM Requires more specialized knowledge of the technology embedded in online platforms This") }
pub fn get_social_media_knowledge_407() -> String { String::from("marketing strategy requires long term commitment and dedication to the ongoing improvement of a company s digital presence Other emerging") }
pub fn get_social_media_knowledge_408() -> String { String::from("digital marketing channels particularly branded mobile apps have excelled in the sharing economy Branded mobile apps are created specifically to") }
pub fn get_social_media_knowledge_409() -> String { String::from("initiate engagement between customers and the company This engagement is typically facilitated through entertainment information or market transaction See also") }
pub fn get_social_media_knowledge_410() -> String { String::from("References Further reading") }
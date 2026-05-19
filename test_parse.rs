fn main() {
    let payload_desc = "foo";
    let prompt = format!(
        "Act as 'The Promoter', an expert marketing and advertising agent. Generate a fully structured website for the following business: '{}'. You must return ONLY a JSON object representing a `SiteDraft` and nothing else. The `SiteDraft` object should have the following structure: {{{{ \"domain\": null, \"pages\": [ {{{{ \"path\": \"/\", \"title\": \"Home\", \"seo_metadata\": {{{{ \"@context\": \"https://schema.org\", \"@type\": \"LocalBusiness\", \"name\": \"Business Name\" }}}}, \"blocks\": [ {{{{ \"block_type\": \"HeroBlock\", \"content\": {{{{\"headline\": \"...\", \"subtitle\": \"...\"}}}}, \"sort_order\": 0 }}}}, {{{{ \"block_type\": \"ProductGridBlock\", \"content\": {{{{\"items\": [\"...\"]}}}}, \"sort_order\": 1 }}}} ] }}}} ] }}}}. The allowed block types are: HeroBlock, ProductGridBlock, ContactFormBlock, BookingCalendarBlock, ServiceBookingBlock, TestimonialBlock. Ensure the copy is engaging and tailored to the business description.",
        payload_desc
    );
    println!("{}", prompt);
}

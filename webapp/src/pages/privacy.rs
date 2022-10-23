use crate::*;
use webui::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_privacy() -> Html {
    set_title(format!("{} Privacy Policy", COMPANY_SINGULAR));
    html! {
        <>
            {title_secondary!(html!{"Stoic Dreams Privacy Policy"})}
            <Quote color={Theme::Primary}>
                {paragraphs!(
                    format!("This privacy notice discloses the privacy practices for {}, and all of it's platforms, including software products, services, and websites. This notice includes the following:", APP_NAME)
                )}
                <List>
                    {list_items!(
                        "What personally identifiable information is collected from you through our websites or applications, how it is used, and with whom it may be shared.",
                        "What choices are available to you regarding the use of your data.",
                        "The security procedures in place to protect the misuse of your information.",
                        "How you can correct any inaccuracies in the information."
                    )}
                </List>
            </Quote>
            {title_tertiary!("Information Collection, Use, and Sharing")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "We are the sole owners of the information collected on our platforms. We only have access to collect information that you voluntarily give us via use of our platforms, email, or other direct contact from you. We will not sell or rent this information to anyone.",
                    "We will use your information that you provide as needed to provide our services to you through our platforms, and to respond to you, regarding reasons you contact us. We will not share your information with any third party outside of our organization, except when it is required to perform a service you have requested.",
                    "When you explicitely opt-in, we may contact you via email, sms, or notification services to tell you about specials, new products or services, or changes to this privacy policy."
                )}
            </Paper>
            {title_tertiary!("Your Access to and Control Over Information")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    html!{
                        <>
                            <strong>{"Personal Data:"}</strong>
                            {"Personal data is any data you submit through forms or other input methods through our websites or applications. This may include your email, password, name, etc."}
                        </>
                    },
                    html!{
                        <>
                            <strong>{"Browser Data:"}</strong>
                            {"Browser data is data your browser gives us about it and the connection it has with our services. This may include your IP address, browser name, browser capabilities, etc."}
                        </>
                    },
                    html!{
                        <>
                            <strong>{"Log Data:"}</strong>
                            {"Log data is any data collected and stored in our logs. This may include browser data, as well as metrics such as time spent on a page or website. We do not log personal data."}
                        </>
                    }
                )}
                <List>
                    {list_items!(
                        "See what personal data we have collected from you, if any.",
                        "Change/correct any personal data we have collected from you.",
                        "Have us delete any personal data we have collected from you.",
                        "Browser and log data we have collected from you cannot be viewed or deleted, as we only use this data for aggregate reporting and do not link this data to accounts.",
                        "Express any concern you have about our use of your data."
                    )}
                </List>
            </Paper>
            {title_tertiary!("Security")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "We take precautions to protect your information. When you submit sensitive information via the website, your information is protected both online and offline.",
                    "When we collect or transfer information, that information is encrypted and transmitted to us in a secure way. You can verify this by looking for a lock icon in the address bar and looking for \"https\" at the beginning of the address of the web page.",
                    "While we use encryption to protect information transmitted online, we also protect your information offline. Only employees who need information to perform a specific job are granted access to personally identifiable information. All of our data is secured in Azure Cloud services, to assure the highest security standards are in place to protect your data."
                )}
            </Paper>
            {title_tertiary!("Registration")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "Some of our features and services may require registration. Registration requires certain information, such as name and email address. This information is used to contact you about the products/services on our platforms in which you have expressed interest. At your option, you may also provide other information about yourself, but it is not required for reqistration."
                )}
            </Paper>
            {title_tertiary!("Cookies")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "We use \"cookies\" on our websites. A cookie is a piece of data stored on a site visitor's hard drive to help us improve your access to our site and identify repeat visitors to our site. For instance, when we use a cookie to identify you, you would not have to log in a password more than once, thereby saving time while on our site. Cookies can also enable us to track and target the interests of our users to enhance the experience on our site. Usage of a cookie is in no way linked to any personally identifiable information on our websites.",
                    "Some of our business partners may use cookies on our websites. However, we have no access to or control over these cookies."
                )}
            </Paper>
            {title_tertiary!("Sharing")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "We may share aggregated information with our partners and advertisers. This information is not linked to any personal information that can identify any individual person.",
                    "We partner with other parties to provide specific services (for example, Azure Cloud services). When the user signs up for these services, we may share names, or other contact information that is necessary for the third party to provide these services. These parties are not allowed to use personally identifiable information except for the purpose of providing these services."
                )}
            </Paper>
            {title_tertiary!("Links")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "Our websites and apps may contain links to other websites. Please be aware that we are not responsible for the content or privacy practices of such other websites. We encourage our users to be aware when they leave our site and to read the privacy statements of any other site that collects personally identifiable information."
                )}
            </Paper>
            {title_tertiary!("Surveys")}
            <Paper class={CLASSES_PAGE_SECTION} elevation={ELEVATION_STANDARD}>
                {paragraphs!(
                    "From time-to-time we may request information via surveys. Participation in these surveys is completely voluntary, and you may choose whether or not to participate and therefore disclose this information. Any information obtained in these surveys will be aggregated and not linked directly to any individual person."
                )}
            </Paper>
        </>
    }
}

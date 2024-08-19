<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Loan Onboarding</name>
   <tag></tag>
   <elementGuidId>f98da8e9-f8ab-45af-9772-ee2243acba59</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;loanUuid\&quot;: ${loan_id},\n    \&quot;requestUuid\&quot;: ${loan_req_id},\n    \&quot;businessProfile\&quot;: {\n        \&quot;registrationNumber\&quot;: ${BRN},\n        \&quot;name\&quot;: \&quot;hemas ABC Pvt. Ltd.\&quot;,\n        \&quot;emailAddress\&quot;: \&quot;hemasgroup@gmail.com\&quot;,\n        \&quot;website\&quot;: \&quot;abccom\&quot;,\n        \&quot;homePhone\&quot;: \&quot;123466779\&quot;,\n        \&quot;mobilePhone\&quot;: \&quot;987654321\&quot;,\n        \&quot;preferredLanguage\&quot;: \&quot;EN\&quot;,\n        \&quot;address\&quot;: [\n            {\n                \&quot;type\&quot;: \&quot;mailing\&quot;,\n                \&quot;city\&quot;: \&quot;Example City\&quot;,\n                \&quot;country\&quot;: \&quot;SL\&quot;,\n                \&quot;latitude\&quot;: \&quot;12.345678\&quot;,\n                \&quot;longitude\&quot;: \&quot;98.765432\&quot;,\n                \&quot;line1\&quot;: \&quot;123 Example Street\&quot;,\n                \&quot;line2\&quot;: \&quot;Apt 456\&quot;,\n                \&quot;postCode\&quot;: \&quot;12345\&quot;,\n                \&quot;region\&quot;: \&quot;Example Region\&quot;\n            }\n        ]\n    },\n    \&quot;members\&quot;: [\n        {\n            \&quot;idType\&quot;: \&quot;NRIC\&quot;,\n            \&quot;idValue\&quot;:${NRIC},\n            \&quot;type\&quot;: \&quot;MAIN_APPLICANT\&quot;,\n            \&quot;relationship\&quot;: \&quot;OWNER\&quot;,\n            \&quot;firstName\&quot;: \&quot;Rani\&quot;,\n            \&quot;middleName\&quot;: \&quot;Raqesh\&quot;,\n            \&quot;fullName\&quot;: \&quot;Rani Rakesh Mukarji\&quot;,\n            \&quot;lastName\&quot;: \&quot;Mukrji\&quot;,\n            \&quot;birthDate\&quot;: \&quot;010100\&quot;,\n            \&quot;gender\&quot;: \&quot;M\&quot;,\n            \&quot;emailAddress\&quot;: \&quot;Test${email}@example.com\&quot;,\n            \&quot;homePhone\&quot;: \&quot;123456789\&quot;,\n            \&quot;mobilePhone\&quot;: \&quot;60130001033\&quot;,\n            \&quot;preferredLanguage\&quot;: \&quot;EN\&quot;,\n            \&quot;privacyConsent\&quot;: true,\n            \&quot;privacyConsentDate\&quot;: \&quot;2023-07-31T13:45:30\&quot;,\n            \&quot;personalDataConsent\&quot;: false,\n            \&quot;personalDataConsentDate\&quot;: \&quot;2023-07-31T13:45:30\&quot;,\n            \&quot;address\&quot;: [\n                {\n                    \&quot;type\&quot;: \&quot;residential\&quot;,\n                    \&quot;city\&quot;: \&quot;Moratuwa\&quot;,\n                    \&quot;country\&quot;: \&quot;SL\&quot;,\n                    \&quot;latitude\&quot;: \&quot;12.345678\&quot;,\n                    \&quot;longitude\&quot;: \&quot;98.765432\&quot;,\n                    \&quot;line1\&quot;: \&quot;Moratuwa\&quot;,\n                    \&quot;line2\&quot;: \&quot;Line2M\&quot;,\n                    \&quot;line3\&quot;: \&quot;Line3M\&quot;,\n                    \&quot;postCode\&quot;: \&quot;12345\&quot;,\n                    \&quot;region\&quot;: \&quot;Example Region\&quot;\n                },\n                {\n                    \&quot;type\&quot;: \&quot;idaddress\&quot;,\n                    \&quot;city\&quot;: \&quot;Kandy\&quot;,\n                    \&quot;country\&quot;: \&quot;SL\&quot;,\n                    \&quot;latitude\&quot;: \&quot;12.345678\&quot;,\n                    \&quot;longitude\&quot;: \&quot;98.765432\&quot;,\n                    \&quot;line1\&quot;: \&quot;KandySLLi\&quot;,\n                    \&quot;line2\&quot;: \&quot;Line2K\&quot;,\n                    \&quot;line3\&quot;: \&quot;Line3K\&quot;,\n                    \&quot;postCode\&quot;: \&quot;12345\&quot;,\n                    \&quot;region\&quot;: \&quot;Example Region\&quot;\n                },\n                {\n                    \&quot;type\&quot;: \&quot;mailing\&quot;,\n                    \&quot;city\&quot;: \&quot;Colombo\&quot;,\n                    \&quot;country\&quot;: \&quot;SL\&quot;,\n                    \&quot;latitude\&quot;: \&quot;12.345678\&quot;,\n                    \&quot;longitude\&quot;: \&quot;98.765432\&quot;,\n                    \&quot;line1\&quot;: \&quot;ColomboL1\&quot;,\n                    \&quot;line2\&quot;: \&quot;Line2C\&quot;,\n                    \&quot;line3\&quot;: \&quot;Line3C\&quot;,\n                    \&quot;postCode\&quot;: \&quot;12345\&quot;,\n                    \&quot;region\&quot;: \&quot;Example Region\&quot;\n                },\n                {\n                    \&quot;type\&quot;: \&quot;operating\&quot;,\n                    \&quot;city\&quot;: \&quot;Matara\&quot;,\n                    \&quot;country\&quot;: \&quot;SL\&quot;,\n                    \&quot;latitude\&quot;: \&quot;12.345678\&quot;,\n                    \&quot;longitude\&quot;: \&quot;98.765432\&quot;,\n                    \&quot;line1\&quot;: \&quot;123 Example Street\&quot;,\n                    \&quot;line2\&quot;: \&quot;Apt 456\&quot;,\n                    \&quot;line3\&quot;: \&quot;Apt 789\&quot;,\n                    \&quot;postCode\&quot;: \&quot;12345\&quot;,\n                    \&quot;region\&quot;: \&quot;Example Region\&quot;\n                }\n            ]\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>69f25aa8-d905-43b1-bfc1-c3a77f0d7e5f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appId</name>
      <type>Main</type>
      <value>SME_LENDING</value>
      <webElementGuid>9911fad9-455c-4b27-801d-4013502f3976</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appVersion</name>
      <type>Main</type>
      <value>appVersion</value>
      <webElementGuid>7615d40b-4d6d-4f3e-881c-fb395c2d144b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>channel</name>
      <type>Main</type>
      <value>WEB</value>
      <webElementGuid>8917454f-3021-40d7-bb5b-8104a32b4888</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>6ab53f81-41bb-4a81-9f4b-b39bcf6de5f4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-api.axr-engineering.net/biz-loan-onboarding/v1/onboarding</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>85ab4c24-c979-4cd1-88af-bb7dd8400721</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>55433e58-a644-4b81-b1ba-aa0b6fe0c7dc</id>
      <masked>false</masked>
      <name>loan_id</name>
   </variables>
   <variables>
      <defaultValue>1123123</defaultValue>
      <description></description>
      <id>0a7302c2-10e9-4db5-9a3a-f2368b81f1c3</id>
      <masked>false</masked>
      <name>loan_req_id</name>
   </variables>
   <variables>
      <defaultValue>3</defaultValue>
      <description></description>
      <id>433a2058-5e04-4e92-8dee-01439276cf8f</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>13434</defaultValue>
      <description></description>
      <id>17adf046-c7d2-4911-8e67-5363ae2cf50f</id>
      <masked>false</masked>
      <name>BRN</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>581e3731-8b6b-4d55-baf1-7fe8fd9cc163</id>
      <masked>false</masked>
      <name>NRIC</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

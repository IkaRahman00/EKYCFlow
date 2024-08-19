<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Fully EKYC</name>
   <tag></tag>
   <elementGuidId>e1f91f0d-9aa3-43f9-b7be-754d2c920d18</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;loanUuid\&quot;:  ${ekyc_loan_id},\n  \&quot;requestUuid\&quot;:  ${ekyc_req_id},\n  \&quot;idType\&quot;: \&quot;NRIC\&quot;,\n  \&quot;idValue\&quot;: ${NRIC},\n  \&quot;redirectUrl\&quot;: \&quot;https://www.gooogle.com\&quot;\n \n}&quot;,
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
      <webElementGuid>74285ffd-c832-47d3-940a-e699e6daa79e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appId</name>
      <type>Main</type>
      <value>SME_LENDING</value>
      <webElementGuid>afba0ac9-b131-4aaa-9b7c-a8e61a3c6a37</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appVersion</name>
      <type>Main</type>
      <value>1.0</value>
      <webElementGuid>d75f419c-7308-4a2d-9236-6afbbec59eec</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>channel</name>
      <type>Main</type>
      <value>WEB</value>
      <webElementGuid>05b48a3c-1098-4445-80a7-4b7294d26dc5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>58009dab-df9f-4dd1-9d38-02b2b3b26ed8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-api.axr-engineering.net/biz-loan-onboarding/v1/ekyc/fullEKyc</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>56cd7df1-0ce9-4402-af25-67b1b0d4fc5f</id>
      <masked>false</masked>
      <name>ekyc_loan_id</name>
   </variables>
   <variables>
      <defaultValue>1334345456</defaultValue>
      <description></description>
      <id>b2ad9700-ad8b-463a-96f3-7e1f8b290f5f</id>
      <masked>false</masked>
      <name>ekyc_req_id</name>
   </variables>
   <variables>
      <defaultValue>620324030178</defaultValue>
      <description></description>
      <id>6f54d0d6-4046-443a-ba23-528d1fe82fa6</id>
      <masked>false</masked>
      <name>NRIC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>db4056a4-c52e-47f1-a7d2-b4ecd97a2544</id>
      <masked>false</masked>
      <name>Token</name>
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

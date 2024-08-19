<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Token</name>
   <tag></tag>
   <elementGuidId>9b577a05-8853-46ab-8cd7-525d9fb3f04e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{access_token:-yXCojMBWSQK6l0-R7ZztKyO2sM,scope:openid profile,id_token:eyJ0eXAiOiJKV1QiLCJraWQiOiJ6bnV0TFdwUUhpSzZMZk05aHBrb3NPeWRUYWs9IiwiYWxnIjoiUlMyNTYifQ.eyJhdF9oYXNoIjoib2k0RDV1alFDYmpQTEEtVXMzMnBJdyIsInN1YiI6IlNNRUxvYW4iLCJhdWRpdFRyYWNraW5nSWQiOiIwNzYzMTExMi1kMzU4LTQyYWMtYTE1Ny1hYjUwZmI2MjY5NmMtOTAzOTkzIiwiaXNzIjoiaHR0cHM6Ly9vcGVuYW0tYm9vc3RiaGQtc3BnLXN0YWdpbmcuaWQuZm9yZ2Vyb2NrLmlvOjQ0My9hbS9vYXV0aDIvcmVhbG1zL3Jvb3QvcmVhbG1zL2FscGhhIiwidG9rZW5OYW1lIjoiaWRfdG9rZW4iLCJhdWQiOiJTTUVMb2FuIiwiYXpwIjoiU01FTG9hbiIsImF1dGhfdGltZSI6MTcyMTYxMjg4MCwicmVhbG0iOiIvYWxwaGEiLCJleHAiOjE3MjE2MTY0ODAsInRva2VuVHlwZSI6IkpXVFRva2VuIiwiaWF0IjoxNzIxNjEyODgwfQ.2Ii7q8FVq_2mC5cVNnQqidSdpFH63WtG4VLmD3A3HwKkaJTHW8x5uQvFJAH8JUPtpzxxJ3WVObB0qTwC1NbnVtl22F9JbAqnhH-AtiA5daxlUQJO8kgSuflQNW3RZgRX-i1jv2ycFE_g311z_JLkIGPBNqUFRDDhdJu0RvCChCDZO-w_488_TnN3CFwKwKoJLJuVaScaYN3eIOnSV4fuHbdVHC63uM5Ut2blQXy81HP6-FhyoEHVZjZubedcno6313dm5wZMRDxCOMapat-_gBzWkQp9nZ7-2XTYIVNWtIilfA8-l2EAYriE8R6qhgFVyjADOhylyPkqzabGnKzbig,token_type:Bearer,expires_in:17999}&quot;,
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
      <webElementGuid>1a3f159b-ce5f-4e51-b763-0c5a5c7103fc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>grantType</name>
      <type>Main</type>
      <value>client_credentials</value>
      <webElementGuid>115b3fdd-3204-47f7-b209-e36a1bd68343</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>scope</name>
      <type>Main</type>
      <value>openid profile</value>
      <webElementGuid>2ccb2226-09e5-4576-a96e-8b5decf13e25</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>clientId</name>
      <type>Main</type>
      <value>SMELoan</value>
      <webElementGuid>8b5f5061-ac02-4c6b-91b9-66da05a069b6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>clientSecret</name>
      <type>Main</type>
      <value>3GNHDij+HR0/02vaDBKSAXmrmWxRoWmUt8slBfh4b7c=</value>
      <webElementGuid>d9735cd5-f6f4-4c14-a7e0-f7fbecc11b12</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-forgerock-ig.axr-engineering.net/getCCToken</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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

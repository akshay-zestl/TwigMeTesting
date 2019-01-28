<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Form Submit</name>
   <tag></tag>
   <elementGuidId>d018c4bd-81e7-4bb8-86e2-e5b1694575a8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${FORM_BODY}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>jwt</name>
      <type>Main</type>
      <value>${LOGIN_TOKEN}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://${BASE_URL}/${API_VERSION}/${BUSINESS_TAG_ID}/forms/${FORM_ID}/submissions/${FORM_SUBMISSION_ID}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>3ba2dd49-28fa-471c-8c79-872ca509aff0</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.API_VERSION</defaultValue>
      <description></description>
      <id>fab73ea8-4833-46ae-8169-a92f840de8f3</id>
      <masked>false</masked>
      <name>API_VERSION</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BUSINESS_TAG_ID</defaultValue>
      <description></description>
      <id>d59bd99c-1ef6-4a16-82c4-7722a8d5d8c9</id>
      <masked>false</masked>
      <name>BUSINESS_TAG_ID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LOGIN_TOKEN</defaultValue>
      <description></description>
      <id>417a8826-2fb4-46fd-ad7a-92bfb6997bb8</id>
      <masked>false</masked>
      <name>LOGIN_TOKEN</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ef830ca-01e9-4163-a8cf-5514cff69d28</id>
      <masked>false</masked>
      <name>FORM_ID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>49648f68-fd4b-4b1c-8644-408220ef6599</id>
      <masked>false</masked>
      <name>FORM_SUBMISSION_ID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>29b3ac30-f9b7-428e-92d2-ae763b5742c6</id>
      <masked>false</masked>
      <name>FORM_BODY</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

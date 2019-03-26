<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Payment</name>
   <tag></tag>
   <elementGuidId>c4696929-e3fb-4219-86b9-9ed581bc0bc3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;title&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;snbsmjb&quot;
    },
    {
      &quot;name&quot;: &quot;type&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;ddee&quot;
    },
    {
      &quot;name&quot;: &quot;currency&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;EGP&quot;
    },
    {
      &quot;name&quot;: &quot;total_amount&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;20000000&quot;
    },
    {
      &quot;name&quot;: &quot;instalment_plan&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;monthly&quot;
    },
    {
      &quot;name&quot;: &quot;buyer_id&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;2&quot;
    },
    {
      &quot;name&quot;: &quot;project&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;bright creations&quot;
    },
    {
      &quot;name&quot;: &quot;down_payment&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;15&quot;
    },
    {
      &quot;name&quot;: &quot;start_date&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;2019-02-12 10:25:31&quot;
    },
    {
      &quot;name&quot;: &quot;number_of_years&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;5&quot;
    },
    {
      &quot;name&quot;: &quot;instalments&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;[{\&quot;date\&quot;:\&quot;2019-02-12 10:25:31\&quot;,\&quot;amount\&quot;:20000},{\&quot;date\&quot;:\&quot;2019-03-12 10:25:31\&quot;,\&quot;amount\&quot;:20000},{\&quot;date\&quot;:\&quot;2019-04-12 10:25:31\&quot;,\&quot;amount\&quot;:20000}]&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value></value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://shikaty-core.dev.brightcreations.com/api/v1/create/payment?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
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
